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
    pub struct AdministrationRegion {
        #[doc = "The election administration body for this area."]
        #[serde(rename = "electionAdministrationBody", default)]
        pub election_administration_body: Option<crate::schemas::AdministrativeBody>,
        #[doc = "An ID for this object. IDs may change in future requests and should not be cached. Access to this field requires special access that can be requested from the Request more link on the Quotas page."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "The city or county that provides election information for this voter. This object can have the same elements as state."]
        #[serde(rename = "local_jurisdiction", default)]
        pub local_jurisdiction: Option<crate::schemas::AdministrationRegion>,
        #[doc = "The name of the jurisdiction."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "A list of sources for this area. If multiple sources are listed the data has been aggregated from those sources."]
        #[serde(rename = "sources", default)]
        pub sources: Option<Vec<crate::schemas::Source>>,
    }
    impl ::field_selector::FieldSelector for AdministrationRegion {
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
    pub struct AdministrativeBody {
        #[doc = "A URL provided by this administrative body for information on absentee voting."]
        #[serde(rename = "absenteeVotingInfoUrl", default)]
        pub absentee_voting_info_url: Option<String>,
        #[serde(rename = "addressLines", default)]
        pub address_lines: Option<Vec<String>>,
        #[doc = "A URL provided by this administrative body to give contest information to the voter."]
        #[serde(rename = "ballotInfoUrl", default)]
        pub ballot_info_url: Option<String>,
        #[doc = "The mailing address of this administrative body."]
        #[serde(rename = "correspondenceAddress", default)]
        pub correspondence_address: Option<crate::schemas::SimpleAddressType>,
        #[doc = "A URL provided by this administrative body for looking up general election information."]
        #[serde(rename = "electionInfoUrl", default)]
        pub election_info_url: Option<String>,
        #[doc = "The election officials for this election administrative body."]
        #[serde(rename = "electionOfficials", default)]
        pub election_officials: Option<Vec<crate::schemas::ElectionOfficial>>,
        #[doc = "A URL provided by this administrative body for confirming that the voter is registered to vote."]
        #[serde(rename = "electionRegistrationConfirmationUrl", default)]
        pub election_registration_confirmation_url: Option<String>,
        #[doc = "A URL provided by this administrative body for looking up how to register to vote."]
        #[serde(rename = "electionRegistrationUrl", default)]
        pub election_registration_url: Option<String>,
        #[doc = "A URL provided by this administrative body describing election rules to the voter."]
        #[serde(rename = "electionRulesUrl", default)]
        pub election_rules_url: Option<String>,
        #[doc = "A description of the hours of operation for this administrative body."]
        #[serde(rename = "hoursOfOperation", default)]
        pub hours_of_operation: Option<String>,
        #[doc = "The name of this election administrative body."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "The physical address of this administrative body."]
        #[serde(rename = "physicalAddress", default)]
        pub physical_address: Option<crate::schemas::SimpleAddressType>,
        #[doc = "A description of the services this administrative body may provide."]
        #[serde(rename = "voter_services", default)]
        pub voter_services: Option<Vec<String>>,
        #[doc = "A URL provided by this administrative body for looking up where to vote."]
        #[serde(rename = "votingLocationFinderUrl", default)]
        pub voting_location_finder_url: Option<String>,
    }
    impl ::field_selector::FieldSelector for AdministrativeBody {
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
    pub struct Candidate {
        #[doc = "The URL for the candidate's campaign web site."]
        #[serde(rename = "candidateUrl", default)]
        pub candidate_url: Option<String>,
        #[doc = "A list of known (social) media channels for this candidate."]
        #[serde(rename = "channels", default)]
        pub channels: Option<Vec<crate::schemas::Channel>>,
        #[doc = "The email address for the candidate's campaign."]
        #[serde(rename = "email", default)]
        pub email: Option<String>,
        #[doc = "The candidate's name. If this is a joint ticket it will indicate the name of the candidate at the top of a ticket followed by a / and that name of candidate at the bottom of the ticket. e.g. \"Mitt Romney / Paul Ryan\""]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "The order the candidate appears on the ballot for this contest."]
        #[serde(rename = "orderOnBallot", default)]
        #[serde(with = "crate::parsed_string")]
        pub order_on_ballot: Option<i64>,
        #[doc = "The full name of the party the candidate is a member of."]
        #[serde(rename = "party", default)]
        pub party: Option<String>,
        #[doc = "The voice phone number for the candidate's campaign office."]
        #[serde(rename = "phone", default)]
        pub phone: Option<String>,
        #[doc = "A URL for a photo of the candidate."]
        #[serde(rename = "photoUrl", default)]
        pub photo_url: Option<String>,
    }
    impl ::field_selector::FieldSelector for Candidate {
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
    pub struct Channel {
        #[doc = "The unique public identifier for the candidate's channel."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "The type of channel. The following is a list of types of channels, but is not exhaustive. More channel types may be added at a later time. One of: GooglePlus, YouTube, Facebook, Twitter"]
        #[serde(rename = "type", default)]
        pub r#type: Option<String>,
    }
    impl ::field_selector::FieldSelector for Channel {
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
    pub struct Contest {
        #[doc = "A number specifying the position of this contest on the voter's ballot."]
        #[serde(rename = "ballotPlacement", default)]
        #[serde(with = "crate::parsed_string")]
        pub ballot_placement: Option<i64>,
        #[doc = "The official title on the ballot for this contest, only where available."]
        #[serde(rename = "ballotTitle", default)]
        pub ballot_title: Option<String>,
        #[doc = "The candidate choices for this contest."]
        #[serde(rename = "candidates", default)]
        pub candidates: Option<Vec<crate::schemas::Candidate>>,
        #[doc = "Information about the electoral district that this contest is in."]
        #[serde(rename = "district", default)]
        pub district: Option<crate::schemas::ElectoralDistrict>,
        #[doc = "A description of any additional eligibility requirements for voting in this contest."]
        #[serde(rename = "electorateSpecifications", default)]
        pub electorate_specifications: Option<String>,
        #[doc = "An ID for this object. IDs may change in future requests and should not be cached. Access to this field requires special access that can be requested from the Request more link on the Quotas page."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "The levels of government of the office for this contest. There may be more than one in cases where a jurisdiction effectively acts at two different levels of government; for example, the mayor of the District of Columbia acts at \"locality\" level, but also effectively at both \"administrative-area-2\" and \"administrative-area-1\"."]
        #[serde(rename = "level", default)]
        pub level: Option<Vec<String>>,
        #[doc = "The number of candidates that will be elected to office in this contest."]
        #[serde(rename = "numberElected", default)]
        #[serde(with = "crate::parsed_string")]
        pub number_elected: Option<i64>,
        #[doc = "The number of candidates that a voter may vote for in this contest."]
        #[serde(rename = "numberVotingFor", default)]
        #[serde(with = "crate::parsed_string")]
        pub number_voting_for: Option<i64>,
        #[doc = "The name of the office for this contest."]
        #[serde(rename = "office", default)]
        pub office: Option<String>,
        #[doc = "If this is a partisan election, the name of the party it is for."]
        #[serde(rename = "primaryParty", default)]
        pub primary_party: Option<String>,
        #[doc = "The type of contest. Usually this will be 'General', 'Primary', or 'Run-off' for contests with candidates. For referenda this will be 'Referendum'. For Retention contests this will typically be 'Retention'."]
        #[serde(rename = "type", default)]
        pub r#type: Option<String>,
        #[doc = "The set of ballot responses for the referendum. A ballot response represents a line on the ballot. Common examples might include \"yes\" or \"no\" for referenda. This field is only populated for contests of type 'Referendum'."]
        #[serde(rename = "referendumBallotResponses", default)]
        pub referendum_ballot_responses: Option<Vec<String>>,
        #[doc = "Specifies a short summary of the referendum that is typically on the ballot below the title but above the text. This field is only populated for contests of type 'Referendum'."]
        #[serde(rename = "referendumBrief", default)]
        pub referendum_brief: Option<String>,
        #[doc = "A statement in opposition to the referendum. It does not necessarily appear on the ballot. This field is only populated for contests of type 'Referendum'."]
        #[serde(rename = "referendumConStatement", default)]
        pub referendum_con_statement: Option<String>,
        #[doc = "Specifies what effect abstaining (not voting) on the proposition will have (i.e. whether abstaining is considered a vote against it). This field is only populated for contests of type 'Referendum'."]
        #[serde(rename = "referendumEffectOfAbstain", default)]
        pub referendum_effect_of_abstain: Option<String>,
        #[doc = "The threshold of votes that the referendum needs in order to pass, e.g. \"two-thirds\". This field is only populated for contests of type 'Referendum'."]
        #[serde(rename = "referendumPassageThreshold", default)]
        pub referendum_passage_threshold: Option<String>,
        #[doc = "A statement in favor of the referendum. It does not necessarily appear on the ballot. This field is only populated for contests of type 'Referendum'."]
        #[serde(rename = "referendumProStatement", default)]
        pub referendum_pro_statement: Option<String>,
        #[doc = "A brief description of the referendum. This field is only populated for contests of type 'Referendum'."]
        #[serde(rename = "referendumSubtitle", default)]
        pub referendum_subtitle: Option<String>,
        #[doc = "The full text of the referendum. This field is only populated for contests of type 'Referendum'."]
        #[serde(rename = "referendumText", default)]
        pub referendum_text: Option<String>,
        #[doc = "The title of the referendum (e.g. 'Proposition 42'). This field is only populated for contests of type 'Referendum'."]
        #[serde(rename = "referendumTitle", default)]
        pub referendum_title: Option<String>,
        #[doc = "A link to the referendum. This field is only populated for contests of type 'Referendum'."]
        #[serde(rename = "referendumUrl", default)]
        pub referendum_url: Option<String>,
        #[doc = "The roles which this office fulfills."]
        #[serde(rename = "roles", default)]
        pub roles: Option<Vec<String>>,
        #[doc = "A list of sources for this contest. If multiple sources are listed, the data has been aggregated from those sources."]
        #[serde(rename = "sources", default)]
        pub sources: Option<Vec<crate::schemas::Source>>,
        #[doc = "\"Yes\" or \"No\" depending on whether this a contest being held outside the normal election cycle."]
        #[serde(rename = "special", default)]
        pub special: Option<String>,
    }
    impl ::field_selector::FieldSelector for Contest {
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
    pub struct ContextParams {
        #[serde(rename = "clientProfile", default)]
        pub client_profile: Option<String>,
    }
    impl ::field_selector::FieldSelector for ContextParams {
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
    pub struct DivisionRepresentativeInfoRequest {
        #[serde(rename = "contextParams", default)]
        pub context_params: Option<crate::schemas::ContextParams>,
    }
    impl ::field_selector::FieldSelector for DivisionRepresentativeInfoRequest {
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
    pub struct DivisionSearchRequest {
        #[serde(rename = "contextParams", default)]
        pub context_params: Option<crate::schemas::ContextParams>,
    }
    impl ::field_selector::FieldSelector for DivisionSearchRequest {
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
    pub struct DivisionSearchResponse {
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"civicinfo#divisionSearchResponse\"."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[serde(rename = "results", default)]
        pub results: Option<Vec<crate::schemas::DivisionSearchResult>>,
    }
    impl ::field_selector::FieldSelector for DivisionSearchResponse {
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
    pub struct DivisionSearchResult {
        #[doc = "Other Open Civic Data identifiers that refer to the same division -- for example, those that refer to other political divisions whose boundaries are defined to be coterminous with this one. For example, ocd-division/country:us/state:wy will include an alias of ocd-division/country:us/state:wy/cd:1, since Wyoming has only one Congressional district."]
        #[serde(rename = "aliases", default)]
        pub aliases: Option<Vec<String>>,
        #[doc = "The name of the division."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "The unique Open Civic Data identifier for this division."]
        #[serde(rename = "ocdId", default)]
        pub ocd_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for DivisionSearchResult {
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
    pub struct Election {
        #[doc = "Day of the election in YYYY-MM-DD format."]
        #[serde(rename = "electionDay", default)]
        pub election_day: Option<String>,
        #[doc = "The unique ID of this election."]
        #[serde(rename = "id", default)]
        #[serde(with = "crate::parsed_string")]
        pub id: Option<i64>,
        #[doc = "A displayable name for the election."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "The political division of the election. Represented as an OCD Division ID. Voters within these political jurisdictions are covered by this election. This is typically a state such as ocd-division/country:us/state:ca or for the midterms or general election the entire US (i.e. ocd-division/country:us)."]
        #[serde(rename = "ocdDivisionId", default)]
        pub ocd_division_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for Election {
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
    pub struct ElectionOfficial {
        #[doc = "The email address of the election official."]
        #[serde(rename = "emailAddress", default)]
        pub email_address: Option<String>,
        #[doc = "The fax number of the election official."]
        #[serde(rename = "faxNumber", default)]
        pub fax_number: Option<String>,
        #[doc = "The full name of the election official."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "The office phone number of the election official."]
        #[serde(rename = "officePhoneNumber", default)]
        pub office_phone_number: Option<String>,
        #[doc = "The title of the election official."]
        #[serde(rename = "title", default)]
        pub title: Option<String>,
    }
    impl ::field_selector::FieldSelector for ElectionOfficial {
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
    pub struct ElectionsQueryRequest {
        #[serde(rename = "contextParams", default)]
        pub context_params: Option<crate::schemas::ContextParams>,
    }
    impl ::field_selector::FieldSelector for ElectionsQueryRequest {
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
    pub struct ElectionsQueryResponse {
        #[doc = "A list of available elections"]
        #[serde(rename = "elections", default)]
        pub elections: Option<Vec<crate::schemas::Election>>,
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"civicinfo#electionsQueryResponse\"."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
    }
    impl ::field_selector::FieldSelector for ElectionsQueryResponse {
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
    pub struct ElectoralDistrict {
        #[doc = "An identifier for this district, relative to its scope. For example, the 34th State Senate district would have id \"34\" and a scope of stateUpper."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[serde(rename = "kgForeignKey", default)]
        pub kg_foreign_key: Option<String>,
        #[doc = "The name of the district."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "The geographic scope of this district. If unspecified the district's geography is not known. One of: national, statewide, congressional, stateUpper, stateLower, countywide, judicial, schoolBoard, cityWide, township, countyCouncil, cityCouncil, ward, special"]
        #[serde(rename = "scope", default)]
        pub scope: Option<String>,
    }
    impl ::field_selector::FieldSelector for ElectoralDistrict {
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
    pub struct FieldMetadataProto {
        #[serde(rename = "internal", default)]
        pub internal: Option<crate::schemas::InternalFieldMetadataProto>,
    }
    impl ::field_selector::FieldSelector for FieldMetadataProto {
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
    pub struct GeographicDivision {
        #[doc = "Any other valid OCD IDs that refer to the same division.\n\nBecause OCD IDs are meant to be human-readable and at least somewhat predictable, there are occasionally several identifiers for a single division. These identifiers are defined to be equivalent to one another, and one is always indicated as the primary identifier. The primary identifier will be returned in ocd_id above, and any other equivalent valid identifiers will be returned in this list.\n\nFor example, if this division's OCD ID is ocd-division/country:us/district:dc, this will contain ocd-division/country:us/state:dc."]
        #[serde(rename = "alsoKnownAs", default)]
        pub also_known_as: Option<Vec<String>>,
        #[doc = "The name of the division."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "List of indices in the offices array, one for each office elected from this division. Will only be present if includeOffices was true (or absent) in the request."]
        #[serde(rename = "officeIndices", default)]
        pub office_indices: Option<Vec<u32>>,
    }
    impl ::field_selector::FieldSelector for GeographicDivision {
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
    pub struct InternalFieldMetadataProto {
        #[serde(rename = "isAuto", default)]
        pub is_auto: Option<bool>,
        #[serde(rename = "sourceSummary", default)]
        pub source_summary: Option<crate::schemas::InternalSourceSummaryProto>,
    }
    impl ::field_selector::FieldSelector for InternalFieldMetadataProto {
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
    pub struct InternalSourceSummaryProto {
        #[serde(rename = "dataset", default)]
        pub dataset: Option<String>,
        #[serde(rename = "provider", default)]
        pub provider: Option<String>,
    }
    impl ::field_selector::FieldSelector for InternalSourceSummaryProto {
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
    pub struct LivegraphBacktraceRecordInfo {
        #[serde(rename = "dataSourcePublishMsec", default)]
        #[serde(with = "crate::parsed_string")]
        pub data_source_publish_msec: Option<i64>,
        #[serde(rename = "expId", default)]
        pub exp_id: Option<String>,
        #[serde(rename = "expInfo", default)]
        pub exp_info: Option<crate::schemas::LivegraphBacktraceRecordInfoExpInfo>,
        #[serde(rename = "isRecon", default)]
        pub is_recon: Option<bool>,
        #[serde(rename = "isWlmThrottled", default)]
        pub is_wlm_throttled: Option<bool>,
        #[serde(rename = "numberOfTriples", default)]
        #[serde(with = "crate::parsed_string")]
        pub number_of_triples: Option<i64>,
        #[serde(rename = "priority", default)]
        pub priority: Option<String>,
        #[serde(rename = "process", default)]
        pub process: Option<String>,
        #[serde(rename = "proxyReceiveMsec", default)]
        #[serde(with = "crate::parsed_string")]
        pub proxy_receive_msec: Option<i64>,
        #[serde(rename = "proxySentMsec", default)]
        #[serde(with = "crate::parsed_string")]
        pub proxy_sent_msec: Option<i64>,
        #[serde(rename = "recordId", default)]
        pub record_id: Option<String>,
        #[serde(rename = "shouldMonitorLatency", default)]
        pub should_monitor_latency: Option<bool>,
        #[serde(rename = "subscriberReceiveMsec", default)]
        #[serde(with = "crate::parsed_string")]
        pub subscriber_receive_msec: Option<i64>,
        #[serde(rename = "topicBuildFinishMsec", default)]
        #[serde(with = "crate::parsed_string")]
        pub topic_build_finish_msec: Option<i64>,
        #[serde(rename = "topicBuildStartMsec", default)]
        #[serde(with = "crate::parsed_string")]
        pub topic_build_start_msec: Option<i64>,
        #[serde(rename = "version", default)]
        pub version: Option<String>,
    }
    impl ::field_selector::FieldSelector for LivegraphBacktraceRecordInfo {
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
    pub struct LivegraphBacktraceRecordInfoExpInfo {
        #[serde(rename = "deletedIns", default)]
        pub deleted_ins: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for LivegraphBacktraceRecordInfoExpInfo {
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
    pub struct MessageSet {
        #[serde(rename = "recordMessageSetExt", default)]
        pub record_message_set_ext: Option<crate::schemas::LivegraphBacktraceRecordInfo>,
    }
    impl ::field_selector::FieldSelector for MessageSet {
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
    pub struct Office {
        #[doc = "The OCD ID of the division with which this office is associated."]
        #[serde(rename = "divisionId", default)]
        pub division_id: Option<String>,
        #[doc = "The levels of government of which this office is part. There may be more than one in cases where a jurisdiction effectively acts at two different levels of government; for example, the mayor of the District of Columbia acts at \"locality\" level, but also effectively at both \"administrative-area-2\" and \"administrative-area-1\"."]
        #[serde(rename = "levels", default)]
        pub levels: Option<Vec<String>>,
        #[doc = "The human-readable name of the office."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "List of indices in the officials array of people who presently hold this office."]
        #[serde(rename = "officialIndices", default)]
        pub official_indices: Option<Vec<u32>>,
        #[doc = "The roles which this office fulfills. Roles are not meant to be exhaustive, or to exactly specify the entire set of responsibilities of a given office, but are meant to be rough categories that are useful for general selection from or sorting of a list of offices."]
        #[serde(rename = "roles", default)]
        pub roles: Option<Vec<String>>,
        #[doc = "A list of sources for this office. If multiple sources are listed, the data has been aggregated from those sources."]
        #[serde(rename = "sources", default)]
        pub sources: Option<Vec<crate::schemas::Source>>,
    }
    impl ::field_selector::FieldSelector for Office {
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
    pub struct Official {
        #[doc = "Addresses at which to contact the official."]
        #[serde(rename = "address", default)]
        pub address: Option<Vec<crate::schemas::SimpleAddressType>>,
        #[doc = "A list of known (social) media channels for this official."]
        #[serde(rename = "channels", default)]
        pub channels: Option<Vec<crate::schemas::Channel>>,
        #[doc = "The direct email addresses for the official."]
        #[serde(rename = "emails", default)]
        pub emails: Option<Vec<String>>,
        #[doc = "The official's name."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "The full name of the party the official belongs to."]
        #[serde(rename = "party", default)]
        pub party: Option<String>,
        #[doc = "The official's public contact phone numbers."]
        #[serde(rename = "phones", default)]
        pub phones: Option<Vec<String>>,
        #[doc = "A URL for a photo of the official."]
        #[serde(rename = "photoUrl", default)]
        pub photo_url: Option<String>,
        #[doc = "The official's public website URLs."]
        #[serde(rename = "urls", default)]
        pub urls: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for Official {
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
    pub struct PointProto {
        #[serde(rename = "latE7", default)]
        pub lat_e7: Option<u32>,
        #[serde(rename = "lngE7", default)]
        pub lng_e7: Option<u32>,
        #[serde(rename = "metadata", default)]
        pub metadata: Option<crate::schemas::FieldMetadataProto>,
        #[serde(rename = "temporaryData", default)]
        pub temporary_data: Option<crate::schemas::MessageSet>,
    }
    impl ::field_selector::FieldSelector for PointProto {
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
    pub struct PollingLocation {
        #[doc = "The address of the location."]
        #[serde(rename = "address", default)]
        pub address: Option<crate::schemas::SimpleAddressType>,
        #[doc = "The last date that this early vote site or drop off location may be used. This field is not populated for polling locations."]
        #[serde(rename = "endDate", default)]
        pub end_date: Option<String>,
        #[doc = "An ID for this object. IDs may change in future requests and should not be cached. Access to this field requires special access that can be requested from the Request more link on the Quotas page."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "Latitude of the location, in degrees north of the equator. Only some locations -- generally, ballot drop boxes for vote-by-mail elections -- will have this set; for others, use a geocoding service like the Google Maps API to resolve the address to a geographic point."]
        #[serde(rename = "latitude", default)]
        pub latitude: Option<f64>,
        #[doc = "Longitude of the location, in degrees east of the Prime Meridian. Only some locations -- generally, ballot drop boxes for vote-by-mail elections -- will have this set; for others, use a geocoding service like the Google Maps API to resolve the address to a geographic point."]
        #[serde(rename = "longitude", default)]
        pub longitude: Option<f64>,
        #[doc = "The name of the early vote site or drop off location. This field is not populated for polling locations."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "Notes about this location (e.g. accessibility ramp or entrance to use)."]
        #[serde(rename = "notes", default)]
        pub notes: Option<String>,
        #[doc = "A description of when this location is open."]
        #[serde(rename = "pollingHours", default)]
        pub polling_hours: Option<String>,
        #[doc = "A list of sources for this location. If multiple sources are listed the data has been aggregated from those sources."]
        #[serde(rename = "sources", default)]
        pub sources: Option<Vec<crate::schemas::Source>>,
        #[doc = "The first date that this early vote site or drop off location may be used. This field is not populated for polling locations."]
        #[serde(rename = "startDate", default)]
        pub start_date: Option<String>,
        #[doc = "The services provided by this early vote site or drop off location. This field is not populated for polling locations."]
        #[serde(rename = "voterServices", default)]
        pub voter_services: Option<String>,
    }
    impl ::field_selector::FieldSelector for PollingLocation {
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
    pub struct PostalAddress {
        #[serde(rename = "addressLines", default)]
        pub address_lines: Option<Vec<String>>,
        #[serde(rename = "administrativeAreaName", default)]
        pub administrative_area_name: Option<String>,
        #[serde(rename = "countryName", default)]
        pub country_name: Option<String>,
        #[serde(rename = "countryNameCode", default)]
        pub country_name_code: Option<String>,
        #[serde(rename = "dependentLocalityName", default)]
        pub dependent_locality_name: Option<String>,
        #[serde(rename = "dependentThoroughfareName", default)]
        pub dependent_thoroughfare_name: Option<String>,
        #[serde(rename = "firmName", default)]
        pub firm_name: Option<String>,
        #[serde(rename = "isDisputed", default)]
        pub is_disputed: Option<bool>,
        #[serde(rename = "languageCode", default)]
        pub language_code: Option<String>,
        #[serde(rename = "localityName", default)]
        pub locality_name: Option<String>,
        #[serde(rename = "postBoxNumber", default)]
        pub post_box_number: Option<String>,
        #[serde(rename = "postalCodeNumber", default)]
        pub postal_code_number: Option<String>,
        #[serde(rename = "postalCodeNumberExtension", default)]
        pub postal_code_number_extension: Option<String>,
        #[serde(rename = "premiseName", default)]
        pub premise_name: Option<String>,
        #[serde(rename = "recipientName", default)]
        pub recipient_name: Option<String>,
        #[serde(rename = "sortingCode", default)]
        pub sorting_code: Option<String>,
        #[serde(rename = "subAdministrativeAreaName", default)]
        pub sub_administrative_area_name: Option<String>,
        #[serde(rename = "subPremiseName", default)]
        pub sub_premise_name: Option<String>,
        #[serde(rename = "thoroughfareName", default)]
        pub thoroughfare_name: Option<String>,
        #[serde(rename = "thoroughfareNumber", default)]
        pub thoroughfare_number: Option<String>,
    }
    impl ::field_selector::FieldSelector for PostalAddress {
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
    pub struct Provenance {
        #[serde(rename = "collidedSegmentSource", default)]
        pub collided_segment_source: Option<crate::schemas::StreetSegmentList>,
        #[serde(rename = "ctclContestUuid", default)]
        pub ctcl_contest_uuid: Option<String>,
        #[serde(rename = "ctclOfficeUuid", default)]
        pub ctcl_office_uuid: Option<String>,
        #[serde(rename = "datasetId", default)]
        #[serde(with = "crate::parsed_string")]
        pub dataset_id: Option<i64>,
        #[serde(rename = "precinctId", default)]
        #[serde(with = "crate::parsed_string")]
        pub precinct_id: Option<i64>,
        #[serde(rename = "precinctSplitId", default)]
        #[serde(with = "crate::parsed_string")]
        pub precinct_split_id: Option<i64>,
        #[serde(rename = "tsStreetSegmentId", default)]
        pub ts_street_segment_id: Option<String>,
        #[serde(rename = "vip5PrecinctId", default)]
        pub vip_5_precinct_id: Option<String>,
        #[serde(rename = "vip5StreetSegmentId", default)]
        pub vip_5_street_segment_id: Option<String>,
        #[serde(rename = "vipStreetSegmentId", default)]
        #[serde(with = "crate::parsed_string")]
        pub vip_street_segment_id: Option<i64>,
    }
    impl ::field_selector::FieldSelector for Provenance {
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
    pub struct RepresentativeInfoData {
        #[doc = "Political geographic divisions that contain the requested address."]
        #[serde(rename = "divisions", default)]
        pub divisions:
            Option<::std::collections::BTreeMap<String, crate::schemas::GeographicDivision>>,
        #[doc = "Elected offices referenced by the divisions listed above. Will only be present if includeOffices was true in the request."]
        #[serde(rename = "offices", default)]
        pub offices: Option<Vec<crate::schemas::Office>>,
        #[doc = "Officials holding the offices listed above. Will only be present if includeOffices was true in the request."]
        #[serde(rename = "officials", default)]
        pub officials: Option<Vec<crate::schemas::Official>>,
    }
    impl ::field_selector::FieldSelector for RepresentativeInfoData {
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
    pub struct RepresentativeInfoRequest {
        #[serde(rename = "contextParams", default)]
        pub context_params: Option<crate::schemas::ContextParams>,
    }
    impl ::field_selector::FieldSelector for RepresentativeInfoRequest {
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
    pub struct RepresentativeInfoResponse {
        #[doc = "Political geographic divisions that contain the requested address."]
        #[serde(rename = "divisions", default)]
        pub divisions:
            Option<::std::collections::BTreeMap<String, crate::schemas::GeographicDivision>>,
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"civicinfo#representativeInfoResponse\"."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The normalized version of the requested address"]
        #[serde(rename = "normalizedInput", default)]
        pub normalized_input: Option<crate::schemas::SimpleAddressType>,
        #[doc = "Elected offices referenced by the divisions listed above. Will only be present if includeOffices was true in the request."]
        #[serde(rename = "offices", default)]
        pub offices: Option<Vec<crate::schemas::Office>>,
        #[doc = "Officials holding the offices listed above. Will only be present if includeOffices was true in the request."]
        #[serde(rename = "officials", default)]
        pub officials: Option<Vec<crate::schemas::Official>>,
    }
    impl ::field_selector::FieldSelector for RepresentativeInfoResponse {
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
    pub struct SimpleAddressType {
        #[doc = "The city or town for the address."]
        #[serde(rename = "city", default)]
        pub city: Option<String>,
        #[doc = "The street name and number of this address."]
        #[serde(rename = "line1", default)]
        pub line_1: Option<String>,
        #[doc = "The second line the address, if needed."]
        #[serde(rename = "line2", default)]
        pub line_2: Option<String>,
        #[doc = "The third line of the address, if needed."]
        #[serde(rename = "line3", default)]
        pub line_3: Option<String>,
        #[doc = "The name of the location."]
        #[serde(rename = "locationName", default)]
        pub location_name: Option<String>,
        #[doc = "The US two letter state abbreviation of the address."]
        #[serde(rename = "state", default)]
        pub state: Option<String>,
        #[doc = "The US Postal Zip Code of the address."]
        #[serde(rename = "zip", default)]
        pub zip: Option<String>,
    }
    impl ::field_selector::FieldSelector for SimpleAddressType {
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
    pub struct Source {
        #[doc = "The name of the data source."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "Whether this data comes from an official government source."]
        #[serde(rename = "official", default)]
        pub official: Option<bool>,
    }
    impl ::field_selector::FieldSelector for Source {
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
    pub struct StreetSegment {
        #[serde(rename = "administrationRegionIds", default)]
        pub administration_region_ids: Option<Vec<String>>,
        #[serde(rename = "beforeGeocodeId", default)]
        pub before_geocode_id: Option<String>,
        #[serde(rename = "catalistUniquePrecinctCode", default)]
        pub catalist_unique_precinct_code: Option<String>,
        #[serde(rename = "city", default)]
        pub city: Option<String>,
        #[serde(rename = "cityCouncilDistrict", default)]
        pub city_council_district: Option<String>,
        #[serde(rename = "congressionalDistrict", default)]
        pub congressional_district: Option<String>,
        #[serde(rename = "contestIds", default)]
        pub contest_ids: Option<Vec<String>>,
        #[serde(rename = "countyCouncilDistrict", default)]
        pub county_council_district: Option<String>,
        #[serde(rename = "countyFips", default)]
        pub county_fips: Option<String>,
        #[serde(rename = "datasetId", default)]
        #[serde(with = "crate::parsed_string")]
        pub dataset_id: Option<i64>,
        #[serde(rename = "earlyVoteSiteByIds", default)]
        pub early_vote_site_by_ids: Option<Vec<String>>,
        #[serde(rename = "endHouseNumber", default)]
        #[serde(with = "crate::parsed_string")]
        pub end_house_number: Option<i64>,
        #[serde(rename = "geocodedPoint", default)]
        pub geocoded_point: Option<crate::schemas::PointProto>,
        #[serde(rename = "geographicDivisionOcdIds", default)]
        pub geographic_division_ocd_ids: Option<Vec<String>>,
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[serde(rename = "judicialDistrict", default)]
        pub judicial_district: Option<String>,
        #[serde(rename = "mailOnly", default)]
        pub mail_only: Option<bool>,
        #[serde(rename = "municipalDistrict", default)]
        pub municipal_district: Option<String>,
        #[serde(rename = "ncoaAddress", default)]
        pub ncoa_address: Option<String>,
        #[serde(rename = "oddOrEvens", default)]
        pub odd_or_evens: Option<Vec<String>>,
        #[serde(rename = "originalId", default)]
        pub original_id: Option<String>,
        #[serde(rename = "pollinglocationByIds", default)]
        pub pollinglocation_by_ids: Option<Vec<String>>,
        #[serde(rename = "precinctName", default)]
        pub precinct_name: Option<String>,
        #[serde(rename = "precinctOcdId", default)]
        pub precinct_ocd_id: Option<String>,
        #[serde(rename = "provenances", default)]
        pub provenances: Option<Vec<crate::schemas::Provenance>>,
        #[serde(rename = "published", default)]
        pub published: Option<bool>,
        #[serde(rename = "schoolDistrict", default)]
        pub school_district: Option<String>,
        #[serde(rename = "startHouseNumber", default)]
        #[serde(with = "crate::parsed_string")]
        pub start_house_number: Option<i64>,
        #[serde(rename = "startLatE7", default)]
        #[serde(with = "crate::parsed_string")]
        pub start_lat_e7: Option<i64>,
        #[serde(rename = "startLngE7", default)]
        #[serde(with = "crate::parsed_string")]
        pub start_lng_e7: Option<i64>,
        #[serde(rename = "state", default)]
        pub state: Option<String>,
        #[serde(rename = "stateHouseDistrict", default)]
        pub state_house_district: Option<String>,
        #[serde(rename = "stateSenateDistrict", default)]
        pub state_senate_district: Option<String>,
        #[serde(rename = "streetName", default)]
        pub street_name: Option<String>,
        #[serde(rename = "subAdministrativeAreaName", default)]
        pub sub_administrative_area_name: Option<String>,
        #[serde(rename = "surrogateId", default)]
        #[serde(with = "crate::parsed_string")]
        pub surrogate_id: Option<i64>,
        #[serde(rename = "targetsmartUniquePrecinctCode", default)]
        pub targetsmart_unique_precinct_code: Option<String>,
        #[serde(rename = "townshipDistrict", default)]
        pub township_district: Option<String>,
        #[serde(rename = "unitNumber", default)]
        pub unit_number: Option<String>,
        #[serde(rename = "unitType", default)]
        pub unit_type: Option<String>,
        #[serde(rename = "vanPrecinctCode", default)]
        pub van_precinct_code: Option<String>,
        #[serde(rename = "voterGeographicDivisionOcdIds", default)]
        pub voter_geographic_division_ocd_ids: Option<Vec<String>>,
        #[serde(rename = "wardDistrict", default)]
        pub ward_district: Option<String>,
        #[serde(rename = "wildcard", default)]
        pub wildcard: Option<bool>,
        #[serde(rename = "zip", default)]
        pub zip: Option<String>,
    }
    impl ::field_selector::FieldSelector for StreetSegment {
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
    pub struct StreetSegmentList {
        #[serde(rename = "segments", default)]
        pub segments: Option<Vec<crate::schemas::StreetSegment>>,
    }
    impl ::field_selector::FieldSelector for StreetSegmentList {
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
    pub struct VoterInfoRequest {
        #[serde(rename = "contextParams", default)]
        pub context_params: Option<crate::schemas::ContextParams>,
        #[serde(rename = "voterInfoSegmentResult", default)]
        pub voter_info_segment_result: Option<crate::schemas::VoterInfoSegmentResult>,
    }
    impl ::field_selector::FieldSelector for VoterInfoRequest {
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
    pub struct VoterInfoResponse {
        #[doc = "Contests that will appear on the voter's ballot."]
        #[serde(rename = "contests", default)]
        pub contests: Option<Vec<crate::schemas::Contest>>,
        #[doc = "Locations where a voter is eligible to drop off a completed ballot. The voter must have received and completed a ballot prior to arriving at the location. The location may not have ballots available on the premises. These locations could be open on or before election day as indicated in the pollingHours field."]
        #[serde(rename = "dropOffLocations", default)]
        pub drop_off_locations: Option<Vec<crate::schemas::PollingLocation>>,
        #[doc = "Locations where the voter is eligible to vote early, prior to election day."]
        #[serde(rename = "earlyVoteSites", default)]
        pub early_vote_sites: Option<Vec<crate::schemas::PollingLocation>>,
        #[doc = "The election that was queried."]
        #[serde(rename = "election", default)]
        pub election: Option<crate::schemas::Election>,
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"civicinfo#voterInfoResponse\"."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "Specifies whether voters in the precinct vote only by mailing their ballots (with the possible option of dropping off their ballots as well)."]
        #[serde(rename = "mailOnly", default)]
        pub mail_only: Option<bool>,
        #[doc = "The normalized version of the requested address"]
        #[serde(rename = "normalizedInput", default)]
        pub normalized_input: Option<crate::schemas::SimpleAddressType>,
        #[doc = "When there are multiple elections for a voter address, the otherElections field is populated in the API response and there are two possibilities: 1. If the earliest election is not the intended election, specify the election ID of the desired election in a second API request using the electionId field. 2. If these elections occur on the same day, the API doesn?t return any polling location, contest, or election official information to ensure that an additional query is made. For user-facing applications, we recommend displaying these elections to the user to disambiguate. A second API request using the electionId field should be made for the election that is relevant to the user."]
        #[serde(rename = "otherElections", default)]
        pub other_elections: Option<Vec<crate::schemas::Election>>,
        #[doc = "Locations where the voter is eligible to vote on election day."]
        #[serde(rename = "pollingLocations", default)]
        pub polling_locations: Option<Vec<crate::schemas::PollingLocation>>,
        #[serde(rename = "precinctId", default)]
        pub precinct_id: Option<String>,
        #[serde(rename = "segments", default)]
        pub segments: Option<Vec<crate::schemas::StreetSegment>>,
        #[doc = "Local Election Information for the state that the voter votes in. For the US, there will only be one element in this array."]
        #[serde(rename = "state", default)]
        pub state: Option<Vec<crate::schemas::AdministrationRegion>>,
    }
    impl ::field_selector::FieldSelector for VoterInfoResponse {
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
    pub struct VoterInfoSegmentResult {
        #[serde(rename = "generatedMillis", default)]
        #[serde(with = "crate::parsed_string")]
        pub generated_millis: Option<i64>,
        #[serde(rename = "postalAddress", default)]
        pub postal_address: Option<crate::schemas::PostalAddress>,
        #[serde(rename = "request", default)]
        pub request: Option<crate::schemas::VoterInfoRequest>,
        #[serde(rename = "response", default)]
        pub response: Option<crate::schemas::VoterInfoResponse>,
    }
    impl ::field_selector::FieldSelector for VoterInfoSegmentResult {
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
    #[doc = "Actions that can be performed on the divisions resource"]
    pub fn divisions(&self) -> crate::divisions::DivisionsActions<A> {
        crate::divisions::DivisionsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the elections resource"]
    pub fn elections(&self) -> crate::elections::ElectionsActions<A> {
        crate::elections::ElectionsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the representatives resource"]
    pub fn representatives(&self) -> crate::representatives::RepresentativesActions<A> {
        crate::representatives::RepresentativesActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
}
pub mod divisions {
    pub mod params {}
    pub struct DivisionsActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> DivisionsActions<'a, A> {
        #[doc = "Searches for political divisions by their natural name or OCD ID."]
        pub fn search(
            &self,
            request: crate::schemas::DivisionSearchRequest,
        ) -> SearchRequestBuilder<A> {
            SearchRequestBuilder {
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
                query: None,
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct SearchRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::DivisionSearchRequest,
        query: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> SearchRequestBuilder<'a, A> {
        #[doc = "The search query. Queries can cover any parts of a OCD ID or a human readable division name. All words given in the query are treated as required patterns. In addition to that, most query operators of the Apache Lucene library are supported. See http://lucene.apache.org/core/2_9_4/queryparsersyntax.html"]
        pub fn query(&mut self, value: impl Into<String>) -> &mut Self {
            self.query = Some(value.into());
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
        ) -> Result<crate::schemas::DivisionSearchResponse, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/civicinfo/v2/".to_owned();
            output.push_str("divisions");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("query", &self.query)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            req
        }
    }
}
pub mod elections {
    pub mod params {}
    pub struct ElectionsActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> ElectionsActions<'a, A> {
        #[doc = "List of available elections to query."]
        pub fn election_query(
            &self,
            request: crate::schemas::ElectionsQueryRequest,
        ) -> ElectionQueryRequestBuilder<A> {
            ElectionQueryRequestBuilder {
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
        #[doc = "Looks up information relevant to a voter based on the voter's registered address."]
        pub fn voter_info_query(
            &self,
            request: crate::schemas::VoterInfoRequest,
            address: impl Into<String>,
        ) -> VoterInfoQueryRequestBuilder<A> {
            VoterInfoQueryRequestBuilder {
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
                address: address.into(),
                election_id: None,
                official_only: None,
                return_all_available_data: None,
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct ElectionQueryRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::ElectionsQueryRequest,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> ElectionQueryRequestBuilder<'a, A> {
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
        ) -> Result<crate::schemas::ElectionsQueryResponse, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/civicinfo/v2/".to_owned();
            output.push_str("elections");
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
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct VoterInfoQueryRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::VoterInfoRequest,
        address: String,
        election_id: Option<i64>,
        official_only: Option<bool>,
        return_all_available_data: Option<bool>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> VoterInfoQueryRequestBuilder<'a, A> {
        #[doc = "The unique ID of the election to look up. A list of election IDs can be obtained at https://www.googleapis.com/civicinfo/{version}/electionsIf no election ID is specified in the query and there is more than one election with data for the given voter, the additional elections are provided in the otherElections response field."]
        pub fn election_id(&mut self, value: i64) -> &mut Self {
            self.election_id = Some(value);
            self
        }
        #[doc = "If set to true, only data from official state sources will be returned."]
        pub fn official_only(&mut self, value: bool) -> &mut Self {
            self.official_only = Some(value);
            self
        }
        #[doc = "If set to true, the query will return the success codeand include any partial information when it is unable to determine a matching address or unable to determine the election for electionId=0 queries."]
        pub fn return_all_available_data(&mut self, value: bool) -> &mut Self {
            self.return_all_available_data = Some(value);
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
        ) -> Result<crate::schemas::VoterInfoResponse, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/civicinfo/v2/".to_owned();
            output.push_str("voterinfo");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("address", &self.address)]);
            let req = req.query(&[("electionId", &self.election_id)]);
            let req = req.query(&[("officialOnly", &self.official_only)]);
            let req = req.query(&[("returnAllAvailableData", &self.return_all_available_data)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            req
        }
    }
}
pub mod representatives {
    pub mod params {
        #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
        pub enum RepresentativeInfoByAddressLevels {
            AdministrativeArea1,
            AdministrativeArea2,
            Country,
            International,
            Locality,
            Regional,
            Special,
            SubLocality1,
            SubLocality2,
        }
        impl RepresentativeInfoByAddressLevels {
            pub fn as_str(self) -> &'static str {
                match self {
                    RepresentativeInfoByAddressLevels::AdministrativeArea1 => "administrativeArea1",
                    RepresentativeInfoByAddressLevels::AdministrativeArea2 => "administrativeArea2",
                    RepresentativeInfoByAddressLevels::Country => "country",
                    RepresentativeInfoByAddressLevels::International => "international",
                    RepresentativeInfoByAddressLevels::Locality => "locality",
                    RepresentativeInfoByAddressLevels::Regional => "regional",
                    RepresentativeInfoByAddressLevels::Special => "special",
                    RepresentativeInfoByAddressLevels::SubLocality1 => "subLocality1",
                    RepresentativeInfoByAddressLevels::SubLocality2 => "subLocality2",
                }
            }
        }
        impl ::std::fmt::Display for RepresentativeInfoByAddressLevels {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
        impl ::serde::Serialize for RepresentativeInfoByAddressLevels {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }
        impl<'de> ::serde::Deserialize<'de> for RepresentativeInfoByAddressLevels {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                let value: &'de str = <&str>::deserialize(deserializer)?;
                Ok(match value {
                    "administrativeArea1" => RepresentativeInfoByAddressLevels::AdministrativeArea1,
                    "administrativeArea2" => RepresentativeInfoByAddressLevels::AdministrativeArea2,
                    "country" => RepresentativeInfoByAddressLevels::Country,
                    "international" => RepresentativeInfoByAddressLevels::International,
                    "locality" => RepresentativeInfoByAddressLevels::Locality,
                    "regional" => RepresentativeInfoByAddressLevels::Regional,
                    "special" => RepresentativeInfoByAddressLevels::Special,
                    "subLocality1" => RepresentativeInfoByAddressLevels::SubLocality1,
                    "subLocality2" => RepresentativeInfoByAddressLevels::SubLocality2,
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
        pub enum RepresentativeInfoByAddressRoles {
            DeputyHeadOfGovernment,
            ExecutiveCouncil,
            GovernmentOfficer,
            HeadOfGovernment,
            HeadOfState,
            HighestCourtJudge,
            Judge,
            LegislatorLowerBody,
            LegislatorUpperBody,
            SchoolBoard,
            SpecialPurposeOfficer,
        }
        impl RepresentativeInfoByAddressRoles {
            pub fn as_str(self) -> &'static str {
                match self {
                    RepresentativeInfoByAddressRoles::DeputyHeadOfGovernment => {
                        "deputyHeadOfGovernment"
                    }
                    RepresentativeInfoByAddressRoles::ExecutiveCouncil => "executiveCouncil",
                    RepresentativeInfoByAddressRoles::GovernmentOfficer => "governmentOfficer",
                    RepresentativeInfoByAddressRoles::HeadOfGovernment => "headOfGovernment",
                    RepresentativeInfoByAddressRoles::HeadOfState => "headOfState",
                    RepresentativeInfoByAddressRoles::HighestCourtJudge => "highestCourtJudge",
                    RepresentativeInfoByAddressRoles::Judge => "judge",
                    RepresentativeInfoByAddressRoles::LegislatorLowerBody => "legislatorLowerBody",
                    RepresentativeInfoByAddressRoles::LegislatorUpperBody => "legislatorUpperBody",
                    RepresentativeInfoByAddressRoles::SchoolBoard => "schoolBoard",
                    RepresentativeInfoByAddressRoles::SpecialPurposeOfficer => {
                        "specialPurposeOfficer"
                    }
                }
            }
        }
        impl ::std::fmt::Display for RepresentativeInfoByAddressRoles {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
        impl ::serde::Serialize for RepresentativeInfoByAddressRoles {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }
        impl<'de> ::serde::Deserialize<'de> for RepresentativeInfoByAddressRoles {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                let value: &'de str = <&str>::deserialize(deserializer)?;
                Ok(match value {
                    "deputyHeadOfGovernment" => {
                        RepresentativeInfoByAddressRoles::DeputyHeadOfGovernment
                    }
                    "executiveCouncil" => RepresentativeInfoByAddressRoles::ExecutiveCouncil,
                    "governmentOfficer" => RepresentativeInfoByAddressRoles::GovernmentOfficer,
                    "headOfGovernment" => RepresentativeInfoByAddressRoles::HeadOfGovernment,
                    "headOfState" => RepresentativeInfoByAddressRoles::HeadOfState,
                    "highestCourtJudge" => RepresentativeInfoByAddressRoles::HighestCourtJudge,
                    "judge" => RepresentativeInfoByAddressRoles::Judge,
                    "legislatorLowerBody" => RepresentativeInfoByAddressRoles::LegislatorLowerBody,
                    "legislatorUpperBody" => RepresentativeInfoByAddressRoles::LegislatorUpperBody,
                    "schoolBoard" => RepresentativeInfoByAddressRoles::SchoolBoard,
                    "specialPurposeOfficer" => {
                        RepresentativeInfoByAddressRoles::SpecialPurposeOfficer
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
        #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
        pub enum RepresentativeInfoByDivisionLevels {
            AdministrativeArea1,
            AdministrativeArea2,
            Country,
            International,
            Locality,
            Regional,
            Special,
            SubLocality1,
            SubLocality2,
        }
        impl RepresentativeInfoByDivisionLevels {
            pub fn as_str(self) -> &'static str {
                match self {
                    RepresentativeInfoByDivisionLevels::AdministrativeArea1 => {
                        "administrativeArea1"
                    }
                    RepresentativeInfoByDivisionLevels::AdministrativeArea2 => {
                        "administrativeArea2"
                    }
                    RepresentativeInfoByDivisionLevels::Country => "country",
                    RepresentativeInfoByDivisionLevels::International => "international",
                    RepresentativeInfoByDivisionLevels::Locality => "locality",
                    RepresentativeInfoByDivisionLevels::Regional => "regional",
                    RepresentativeInfoByDivisionLevels::Special => "special",
                    RepresentativeInfoByDivisionLevels::SubLocality1 => "subLocality1",
                    RepresentativeInfoByDivisionLevels::SubLocality2 => "subLocality2",
                }
            }
        }
        impl ::std::fmt::Display for RepresentativeInfoByDivisionLevels {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
        impl ::serde::Serialize for RepresentativeInfoByDivisionLevels {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }
        impl<'de> ::serde::Deserialize<'de> for RepresentativeInfoByDivisionLevels {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                let value: &'de str = <&str>::deserialize(deserializer)?;
                Ok(match value {
                    "administrativeArea1" => {
                        RepresentativeInfoByDivisionLevels::AdministrativeArea1
                    }
                    "administrativeArea2" => {
                        RepresentativeInfoByDivisionLevels::AdministrativeArea2
                    }
                    "country" => RepresentativeInfoByDivisionLevels::Country,
                    "international" => RepresentativeInfoByDivisionLevels::International,
                    "locality" => RepresentativeInfoByDivisionLevels::Locality,
                    "regional" => RepresentativeInfoByDivisionLevels::Regional,
                    "special" => RepresentativeInfoByDivisionLevels::Special,
                    "subLocality1" => RepresentativeInfoByDivisionLevels::SubLocality1,
                    "subLocality2" => RepresentativeInfoByDivisionLevels::SubLocality2,
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
        pub enum RepresentativeInfoByDivisionRoles {
            DeputyHeadOfGovernment,
            ExecutiveCouncil,
            GovernmentOfficer,
            HeadOfGovernment,
            HeadOfState,
            HighestCourtJudge,
            Judge,
            LegislatorLowerBody,
            LegislatorUpperBody,
            SchoolBoard,
            SpecialPurposeOfficer,
        }
        impl RepresentativeInfoByDivisionRoles {
            pub fn as_str(self) -> &'static str {
                match self {
                    RepresentativeInfoByDivisionRoles::DeputyHeadOfGovernment => {
                        "deputyHeadOfGovernment"
                    }
                    RepresentativeInfoByDivisionRoles::ExecutiveCouncil => "executiveCouncil",
                    RepresentativeInfoByDivisionRoles::GovernmentOfficer => "governmentOfficer",
                    RepresentativeInfoByDivisionRoles::HeadOfGovernment => "headOfGovernment",
                    RepresentativeInfoByDivisionRoles::HeadOfState => "headOfState",
                    RepresentativeInfoByDivisionRoles::HighestCourtJudge => "highestCourtJudge",
                    RepresentativeInfoByDivisionRoles::Judge => "judge",
                    RepresentativeInfoByDivisionRoles::LegislatorLowerBody => "legislatorLowerBody",
                    RepresentativeInfoByDivisionRoles::LegislatorUpperBody => "legislatorUpperBody",
                    RepresentativeInfoByDivisionRoles::SchoolBoard => "schoolBoard",
                    RepresentativeInfoByDivisionRoles::SpecialPurposeOfficer => {
                        "specialPurposeOfficer"
                    }
                }
            }
        }
        impl ::std::fmt::Display for RepresentativeInfoByDivisionRoles {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
        impl ::serde::Serialize for RepresentativeInfoByDivisionRoles {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }
        impl<'de> ::serde::Deserialize<'de> for RepresentativeInfoByDivisionRoles {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                let value: &'de str = <&str>::deserialize(deserializer)?;
                Ok(match value {
                    "deputyHeadOfGovernment" => {
                        RepresentativeInfoByDivisionRoles::DeputyHeadOfGovernment
                    }
                    "executiveCouncil" => RepresentativeInfoByDivisionRoles::ExecutiveCouncil,
                    "governmentOfficer" => RepresentativeInfoByDivisionRoles::GovernmentOfficer,
                    "headOfGovernment" => RepresentativeInfoByDivisionRoles::HeadOfGovernment,
                    "headOfState" => RepresentativeInfoByDivisionRoles::HeadOfState,
                    "highestCourtJudge" => RepresentativeInfoByDivisionRoles::HighestCourtJudge,
                    "judge" => RepresentativeInfoByDivisionRoles::Judge,
                    "legislatorLowerBody" => RepresentativeInfoByDivisionRoles::LegislatorLowerBody,
                    "legislatorUpperBody" => RepresentativeInfoByDivisionRoles::LegislatorUpperBody,
                    "schoolBoard" => RepresentativeInfoByDivisionRoles::SchoolBoard,
                    "specialPurposeOfficer" => {
                        RepresentativeInfoByDivisionRoles::SpecialPurposeOfficer
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
    }
    pub struct RepresentativesActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> RepresentativesActions<'a, A> {
        #[doc = "Looks up political geography and representative information for a single address."]
        pub fn representative_info_by_address(
            &self,
            request: crate::schemas::RepresentativeInfoRequest,
        ) -> RepresentativeInfoByAddressRequestBuilder<A> {
            RepresentativeInfoByAddressRequestBuilder {
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
                address: None,
                include_offices: None,
                levels: None,
                roles: None,
            }
        }
        #[doc = "Looks up representative information for a single geographic division."]
        pub fn representative_info_by_division(
            &self,
            request: crate::schemas::DivisionRepresentativeInfoRequest,
            ocd_id: impl Into<String>,
        ) -> RepresentativeInfoByDivisionRequestBuilder<A> {
            RepresentativeInfoByDivisionRequestBuilder {
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
                ocd_id: ocd_id.into(),
                levels: None,
                recursive: None,
                roles: None,
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct RepresentativeInfoByAddressRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::RepresentativeInfoRequest,
        address: Option<String>,
        include_offices: Option<bool>,
        levels: Option<crate::representatives::params::RepresentativeInfoByAddressLevels>,
        roles: Option<crate::representatives::params::RepresentativeInfoByAddressRoles>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> RepresentativeInfoByAddressRequestBuilder<'a, A> {
        #[doc = "The address to look up. May only be specified if the field ocdId is not given in the URL."]
        pub fn address(&mut self, value: impl Into<String>) -> &mut Self {
            self.address = Some(value.into());
            self
        }
        #[doc = "Whether to return information about offices and officials. If false, only the top-level district information will be returned."]
        pub fn include_offices(&mut self, value: bool) -> &mut Self {
            self.include_offices = Some(value);
            self
        }
        #[doc = "A list of office levels to filter by. Only offices that serve at least one of these levels will be returned. Divisions that don't contain a matching office will not be returned."]
        pub fn levels(
            &mut self,
            value: crate::representatives::params::RepresentativeInfoByAddressLevels,
        ) -> &mut Self {
            self.levels = Some(value);
            self
        }
        #[doc = "A list of office roles to filter by. Only offices fulfilling one of these roles will be returned. Divisions that don't contain a matching office will not be returned."]
        pub fn roles(
            &mut self,
            value: crate::representatives::params::RepresentativeInfoByAddressRoles,
        ) -> &mut Self {
            self.roles = Some(value);
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
        ) -> Result<crate::schemas::RepresentativeInfoResponse, Box<dyn ::std::error::Error>>
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
            let mut output = "https://www.googleapis.com/civicinfo/v2/".to_owned();
            output.push_str("representatives");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("address", &self.address)]);
            let req = req.query(&[("includeOffices", &self.include_offices)]);
            let req = req.query(&[("levels", &self.levels)]);
            let req = req.query(&[("roles", &self.roles)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct RepresentativeInfoByDivisionRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::DivisionRepresentativeInfoRequest,
        ocd_id: String,
        levels: Option<crate::representatives::params::RepresentativeInfoByDivisionLevels>,
        recursive: Option<bool>,
        roles: Option<crate::representatives::params::RepresentativeInfoByDivisionRoles>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> RepresentativeInfoByDivisionRequestBuilder<'a, A> {
        #[doc = "A list of office levels to filter by. Only offices that serve at least one of these levels will be returned. Divisions that don't contain a matching office will not be returned."]
        pub fn levels(
            &mut self,
            value: crate::representatives::params::RepresentativeInfoByDivisionLevels,
        ) -> &mut Self {
            self.levels = Some(value);
            self
        }
        #[doc = "If true, information about all divisions contained in the division requested will be included as well. For example, if querying ocd-division/country:us/district:dc, this would also return all DC's wards and ANCs."]
        pub fn recursive(&mut self, value: bool) -> &mut Self {
            self.recursive = Some(value);
            self
        }
        #[doc = "A list of office roles to filter by. Only offices fulfilling one of these roles will be returned. Divisions that don't contain a matching office will not be returned."]
        pub fn roles(
            &mut self,
            value: crate::representatives::params::RepresentativeInfoByDivisionRoles,
        ) -> &mut Self {
            self.roles = Some(value);
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
        ) -> Result<crate::schemas::RepresentativeInfoData, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/civicinfo/v2/".to_owned();
            output.push_str("representatives/");
            output.push_str(&self.ocd_id);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("levels", &self.levels)]);
            let req = req.query(&[("recursive", &self.recursive)]);
            let req = req.query(&[("roles", &self.roles)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
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
