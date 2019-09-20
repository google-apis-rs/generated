use clap::{App, AppSettings, Arg, SubCommand};
use default_boxed::DefaultBoxed;

#[derive(DefaultBoxed)]
struct Outer<'a, 'b> {
    inner: HeapApp<'a, 'b>,
}

struct HeapApp<'a, 'b> {
    app: App<'a, 'b>,
}

impl<'a, 'b> Default for HeapApp<'a, 'b> {
    fn default() -> Self {
        let mut app = App::new("dfareporting3d1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20190911")
            .about("Manages your DoubleClick Campaign Manager ad campaigns and reports.")
            .after_help("All documentation details can be found at <TODO figure out URL>")
            .arg(Arg::with_name("scope")
                .long("scope")
                .help("Specify the authentication method should be executed in. Each scope requires the user to grant this application permission to use it. If unset, it defaults to the shortest scope url for a particular method.")
                .multiple(true)
                .takes_value(true))
            .arg(Arg::with_name("folder")
                .long("config-dir")
                .help("A directory into which we will store our persistent data. Defaults to a user-writable directory that we will create during the first invocation." )
                .multiple(false)
                .takes_value(true))
            .arg(Arg::with_name("debug")
                .long("debug")
                .help("Provide more output to aid with debugging")
                .multiple(false)
                .takes_value(false));
        let mut account_active_ad_summaries0 = SubCommand::with_name("account_active_ad_summaries")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets the account\'s active ad summary by account ID.");
            account_active_ad_summaries0 = account_active_ad_summaries0.subcommand(mcmd);
        }
        let mut account_permission_groups0 = SubCommand::with_name("account_permission_groups")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets one account permission group by ID.");
            account_permission_groups0 = account_permission_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves the list of account permission groups.");
            account_permission_groups0 = account_permission_groups0.subcommand(mcmd);
        }
        let mut account_permissions0 = SubCommand::with_name("account_permissions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets one account permission by ID.");
            account_permissions0 = account_permissions0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Retrieves the list of account permissions.");
            account_permissions0 = account_permissions0.subcommand(mcmd);
        }
        let mut account_user_profiles0 = SubCommand::with_name("account_user_profiles")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("get").about("Gets one account user profile by ID.");
            account_user_profiles0 = account_user_profiles0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Inserts a new account user profile.");
            account_user_profiles0 = account_user_profiles0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves a list of account user profiles, possibly filtered. This method supports paging.");
            account_user_profiles0 = account_user_profiles0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about(
                "Updates an existing account user profile. This method supports patch semantics.",
            );
            account_user_profiles0 = account_user_profiles0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("update").about("Updates an existing account user profile.");
            account_user_profiles0 = account_user_profiles0.subcommand(mcmd);
        }
        let mut accounts0 = SubCommand::with_name("accounts")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, list, patch and update");
        {
            let mcmd = SubCommand::with_name("get").about("Gets one account by ID.");
            accounts0 = accounts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Retrieves the list of accounts, possibly filtered. This method supports paging.",
            );
            accounts0 = accounts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates an existing account. This method supports patch semantics.");
            accounts0 = accounts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates an existing account.");
            accounts0 = accounts0.subcommand(mcmd);
        }
        let mut ads0 = SubCommand::with_name("ads")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("get").about("Gets one ad by ID.");
            ads0 = ads0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Inserts a new ad.");
            ads0 = ads0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a list of ads, possibly filtered. This method supports paging.");
            ads0 = ads0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates an existing ad. This method supports patch semantics.");
            ads0 = ads0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates an existing ad.");
            ads0 = ads0.subcommand(mcmd);
        }
        let mut advertiser_groups0 = SubCommand::with_name("advertiser_groups")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, patch and update");
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes an existing advertiser group.");
            advertiser_groups0 = advertiser_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets one advertiser group by ID.");
            advertiser_groups0 = advertiser_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Inserts a new advertiser group.");
            advertiser_groups0 = advertiser_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves a list of advertiser groups, possibly filtered. This method supports paging.");
            advertiser_groups0 = advertiser_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about(
                "Updates an existing advertiser group. This method supports patch semantics.",
            );
            advertiser_groups0 = advertiser_groups0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("update").about("Updates an existing advertiser group.");
            advertiser_groups0 = advertiser_groups0.subcommand(mcmd);
        }
        let mut advertiser_landing_pages0 = SubCommand::with_name("advertiser_landing_pages")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("get").about("Gets one landing page by ID.");
            advertiser_landing_pages0 = advertiser_landing_pages0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Inserts a new landing page.");
            advertiser_landing_pages0 = advertiser_landing_pages0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves a list of landing pages.");
            advertiser_landing_pages0 = advertiser_landing_pages0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates an existing landing page. This method supports patch semantics.");
            advertiser_landing_pages0 = advertiser_landing_pages0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates an existing landing page.");
            advertiser_landing_pages0 = advertiser_landing_pages0.subcommand(mcmd);
        }
        let mut advertisers0 = SubCommand::with_name("advertisers")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("get").about("Gets one advertiser by ID.");
            advertisers0 = advertisers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Inserts a new advertiser.");
            advertisers0 = advertisers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Retrieves a list of advertisers, possibly filtered. This method supports paging.",
            );
            advertisers0 = advertisers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates an existing advertiser. This method supports patch semantics.");
            advertisers0 = advertisers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates an existing advertiser.");
            advertisers0 = advertisers0.subcommand(mcmd);
        }
        let mut browsers0 = SubCommand::with_name("browsers")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves a list of browsers.");
            browsers0 = browsers0.subcommand(mcmd);
        }
        let mut campaign_creative_associations0 =
            SubCommand::with_name("campaign_creative_associations")
                .setting(AppSettings::ColoredHelp)
                .about("methods: insert and list");
        {
            let mcmd = SubCommand::with_name("insert").about("Associates a creative with the specified campaign. This method creates a default ad with dimensions matching the creative in the campaign if such a default ad does not exist already.");
            campaign_creative_associations0 = campaign_creative_associations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves the list of creative IDs associated with the specified campaign. This method supports paging.");
            campaign_creative_associations0 = campaign_creative_associations0.subcommand(mcmd);
        }
        let mut campaigns0 = SubCommand::with_name("campaigns")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("get").about("Gets one campaign by ID.");
            campaigns0 = campaigns0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Inserts a new campaign.");
            campaigns0 = campaigns0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Retrieves a list of campaigns, possibly filtered. This method supports paging.",
            );
            campaigns0 = campaigns0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates an existing campaign. This method supports patch semantics.");
            campaigns0 = campaigns0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates an existing campaign.");
            campaigns0 = campaigns0.subcommand(mcmd);
        }
        let mut change_logs0 = SubCommand::with_name("change_logs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets one change log by ID.");
            change_logs0 = change_logs0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a list of change logs. This method supports paging.");
            change_logs0 = change_logs0.subcommand(mcmd);
        }
        let mut cities0 = SubCommand::with_name("cities")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a list of cities, possibly filtered.");
            cities0 = cities0.subcommand(mcmd);
        }
        let mut connection_types0 = SubCommand::with_name("connection_types")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets one connection type by ID.");
            connection_types0 = connection_types0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves a list of connection types.");
            connection_types0 = connection_types0.subcommand(mcmd);
        }
        let mut content_categories0 = SubCommand::with_name("content_categories")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, patch and update");
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes an existing content category.");
            content_categories0 = content_categories0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets one content category by ID.");
            content_categories0 = content_categories0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Inserts a new content category.");
            content_categories0 = content_categories0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves a list of content categories, possibly filtered. This method supports paging.");
            content_categories0 = content_categories0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about(
                "Updates an existing content category. This method supports patch semantics.",
            );
            content_categories0 = content_categories0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("update").about("Updates an existing content category.");
            content_categories0 = content_categories0.subcommand(mcmd);
        }
        let mut conversions0 = SubCommand::with_name("conversions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: batchinsert and batchupdate");
        {
            let mcmd = SubCommand::with_name("batchinsert").about("Inserts conversions.");
            conversions0 = conversions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("batchupdate").about("Updates existing conversions.");
            conversions0 = conversions0.subcommand(mcmd);
        }
        let mut countries0 = SubCommand::with_name("countries")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets one country by ID.");
            countries0 = countries0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves a list of countries.");
            countries0 = countries0.subcommand(mcmd);
        }
        let mut creative_assets0 = SubCommand::with_name("creative_assets")
            .setting(AppSettings::ColoredHelp)
            .about("methods: insert");
        {
            let mcmd = SubCommand::with_name("insert").about("Inserts a new creative asset.");
            creative_assets0 = creative_assets0.subcommand(mcmd);
        }
        let mut creative_field_values0 = SubCommand::with_name("creative_field_values")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, patch and update");
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes an existing creative field value.");
            creative_field_values0 = creative_field_values0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets one creative field value by ID.");
            creative_field_values0 = creative_field_values0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Inserts a new creative field value.");
            creative_field_values0 = creative_field_values0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves a list of creative field values, possibly filtered. This method supports paging.");
            creative_field_values0 = creative_field_values0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about(
                "Updates an existing creative field value. This method supports patch semantics.",
            );
            creative_field_values0 = creative_field_values0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("update").about("Updates an existing creative field value.");
            creative_field_values0 = creative_field_values0.subcommand(mcmd);
        }
        let mut creative_fields0 = SubCommand::with_name("creative_fields")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an existing creative field.");
            creative_fields0 = creative_fields0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets one creative field by ID.");
            creative_fields0 = creative_fields0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Inserts a new creative field.");
            creative_fields0 = creative_fields0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves a list of creative fields, possibly filtered. This method supports paging.");
            creative_fields0 = creative_fields0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates an existing creative field. This method supports patch semantics.");
            creative_fields0 = creative_fields0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates an existing creative field.");
            creative_fields0 = creative_fields0.subcommand(mcmd);
        }
        let mut creative_groups0 = SubCommand::with_name("creative_groups")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("get").about("Gets one creative group by ID.");
            creative_groups0 = creative_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Inserts a new creative group.");
            creative_groups0 = creative_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves a list of creative groups, possibly filtered. This method supports paging.");
            creative_groups0 = creative_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates an existing creative group. This method supports patch semantics.");
            creative_groups0 = creative_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates an existing creative group.");
            creative_groups0 = creative_groups0.subcommand(mcmd);
        }
        let mut creatives0 = SubCommand::with_name("creatives")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("get").about("Gets one creative by ID.");
            creatives0 = creatives0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Inserts a new creative.");
            creatives0 = creatives0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Retrieves a list of creatives, possibly filtered. This method supports paging.",
            );
            creatives0 = creatives0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates an existing creative. This method supports patch semantics.");
            creatives0 = creatives0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates an existing creative.");
            creatives0 = creatives0.subcommand(mcmd);
        }
        let mut dimension_values0 = SubCommand::with_name("dimension_values")
            .setting(AppSettings::ColoredHelp)
            .about("methods: query");
        {
            let mcmd = SubCommand::with_name("query")
                .about("Retrieves list of report dimension values for a list of filters.");
            dimension_values0 = dimension_values0.subcommand(mcmd);
        }
        let mut directory_sites0 = SubCommand::with_name("directory_sites")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, insert and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets one directory site by ID.");
            directory_sites0 = directory_sites0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Inserts a new directory site.");
            directory_sites0 = directory_sites0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves a list of directory sites, possibly filtered. This method supports paging.");
            directory_sites0 = directory_sites0.subcommand(mcmd);
        }
        let mut dynamic_targeting_keys0 = SubCommand::with_name("dynamic_targeting_keys")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, insert and list");
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes an existing dynamic targeting key.");
            dynamic_targeting_keys0 = dynamic_targeting_keys0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Inserts a new dynamic targeting key. Keys must be created at the advertiser level before being assigned to the advertiser\'s ads, creatives, or placements. There is a maximum of 1000 keys per advertiser, out of which a maximum of 20 keys can be assigned per ad, creative, or placement.");
            dynamic_targeting_keys0 = dynamic_targeting_keys0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Retrieves a list of dynamic targeting keys.");
            dynamic_targeting_keys0 = dynamic_targeting_keys0.subcommand(mcmd);
        }
        let mut event_tags0 = SubCommand::with_name("event_tags")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an existing event tag.");
            event_tags0 = event_tags0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets one event tag by ID.");
            event_tags0 = event_tags0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Inserts a new event tag.");
            event_tags0 = event_tags0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a list of event tags, possibly filtered.");
            event_tags0 = event_tags0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates an existing event tag. This method supports patch semantics.");
            event_tags0 = event_tags0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates an existing event tag.");
            event_tags0 = event_tags0.subcommand(mcmd);
        }
        let mut files0 = SubCommand::with_name("files")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves a report file by its report ID and file ID. This method supports media download.");
            files0 = files0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists files for a user profile.");
            files0 = files0.subcommand(mcmd);
        }
        let mut floodlight_activities0 = SubCommand::with_name("floodlight_activities")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, generatetag, get, insert, list, patch and update");
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes an existing floodlight activity.");
            floodlight_activities0 = floodlight_activities0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("generatetag")
                .about("Generates a tag for a floodlight activity.");
            floodlight_activities0 = floodlight_activities0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets one floodlight activity by ID.");
            floodlight_activities0 = floodlight_activities0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Inserts a new floodlight activity.");
            floodlight_activities0 = floodlight_activities0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves a list of floodlight activities, possibly filtered. This method supports paging.");
            floodlight_activities0 = floodlight_activities0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about(
                "Updates an existing floodlight activity. This method supports patch semantics.",
            );
            floodlight_activities0 = floodlight_activities0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("update").about("Updates an existing floodlight activity.");
            floodlight_activities0 = floodlight_activities0.subcommand(mcmd);
        }
        let mut floodlight_activity_groups0 = SubCommand::with_name("floodlight_activity_groups")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, insert, list, patch and update");
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets one floodlight activity group by ID.");
            floodlight_activity_groups0 = floodlight_activity_groups0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("insert").about("Inserts a new floodlight activity group.");
            floodlight_activity_groups0 = floodlight_activity_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves a list of floodlight activity groups, possibly filtered. This method supports paging.");
            floodlight_activity_groups0 = floodlight_activity_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an existing floodlight activity group. This method supports patch semantics.");
            floodlight_activity_groups0 = floodlight_activity_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update")
                .about("Updates an existing floodlight activity group.");
            floodlight_activity_groups0 = floodlight_activity_groups0.subcommand(mcmd);
        }
        let mut floodlight_configurations0 = SubCommand::with_name("floodlight_configurations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, list, patch and update");
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets one floodlight configuration by ID.");
            floodlight_configurations0 = floodlight_configurations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a list of floodlight configurations, possibly filtered.");
            floodlight_configurations0 = floodlight_configurations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an existing floodlight configuration. This method supports patch semantics.");
            floodlight_configurations0 = floodlight_configurations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update")
                .about("Updates an existing floodlight configuration.");
            floodlight_configurations0 = floodlight_configurations0.subcommand(mcmd);
        }
        let mut inventory_items0 = SubCommand::with_name("inventory_items")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets one inventory item by ID.");
            inventory_items0 = inventory_items0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves a list of inventory items, possibly filtered. This method supports paging.");
            inventory_items0 = inventory_items0.subcommand(mcmd);
        }
        let mut languages0 = SubCommand::with_name("languages")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves a list of languages.");
            languages0 = languages0.subcommand(mcmd);
        }
        let mut metros0 = SubCommand::with_name("metros")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves a list of metros.");
            metros0 = metros0.subcommand(mcmd);
        }
        let mut mobile_apps0 = SubCommand::with_name("mobile_apps")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets one mobile app by ID.");
            mobile_apps0 = mobile_apps0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Retrieves list of available mobile apps.");
            mobile_apps0 = mobile_apps0.subcommand(mcmd);
        }
        let mut mobile_carriers0 = SubCommand::with_name("mobile_carriers")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets one mobile carrier by ID.");
            mobile_carriers0 = mobile_carriers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves a list of mobile carriers.");
            mobile_carriers0 = mobile_carriers0.subcommand(mcmd);
        }
        let mut operating_system_versions0 = SubCommand::with_name("operating_system_versions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets one operating system version by ID.");
            operating_system_versions0 = operating_system_versions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a list of operating system versions.");
            operating_system_versions0 = operating_system_versions0.subcommand(mcmd);
        }
        let mut operating_systems0 = SubCommand::with_name("operating_systems")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets one operating system by DART ID.");
            operating_systems0 = operating_systems0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Retrieves a list of operating systems.");
            operating_systems0 = operating_systems0.subcommand(mcmd);
        }
        let mut order_documents0 = SubCommand::with_name("order_documents")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets one order document by ID.");
            order_documents0 = order_documents0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves a list of order documents, possibly filtered. This method supports paging.");
            order_documents0 = order_documents0.subcommand(mcmd);
        }
        let mut orders0 = SubCommand::with_name("orders")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets one order by ID.");
            orders0 = orders0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Retrieves a list of orders, possibly filtered. This method supports paging.",
            );
            orders0 = orders0.subcommand(mcmd);
        }
        let mut placement_groups0 = SubCommand::with_name("placement_groups")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("get").about("Gets one placement group by ID.");
            placement_groups0 = placement_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Inserts a new placement group.");
            placement_groups0 = placement_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves a list of placement groups, possibly filtered. This method supports paging.");
            placement_groups0 = placement_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about(
                "Updates an existing placement group. This method supports patch semantics.",
            );
            placement_groups0 = placement_groups0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("update").about("Updates an existing placement group.");
            placement_groups0 = placement_groups0.subcommand(mcmd);
        }
        let mut placement_strategies0 = SubCommand::with_name("placement_strategies")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, patch and update");
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes an existing placement strategy.");
            placement_strategies0 = placement_strategies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets one placement strategy by ID.");
            placement_strategies0 = placement_strategies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Inserts a new placement strategy.");
            placement_strategies0 = placement_strategies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves a list of placement strategies, possibly filtered. This method supports paging.");
            placement_strategies0 = placement_strategies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about(
                "Updates an existing placement strategy. This method supports patch semantics.",
            );
            placement_strategies0 = placement_strategies0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("update").about("Updates an existing placement strategy.");
            placement_strategies0 = placement_strategies0.subcommand(mcmd);
        }
        let mut placements0 = SubCommand::with_name("placements")
            .setting(AppSettings::ColoredHelp)
            .about("methods: generatetags, get, insert, list, patch and update");
        {
            let mcmd =
                SubCommand::with_name("generatetags").about("Generates tags for a placement.");
            placements0 = placements0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets one placement by ID.");
            placements0 = placements0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Inserts a new placement.");
            placements0 = placements0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Retrieves a list of placements, possibly filtered. This method supports paging.",
            );
            placements0 = placements0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates an existing placement. This method supports patch semantics.");
            placements0 = placements0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates an existing placement.");
            placements0 = placements0.subcommand(mcmd);
        }
        let mut platform_types0 = SubCommand::with_name("platform_types")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets one platform type by ID.");
            platform_types0 = platform_types0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves a list of platform types.");
            platform_types0 = platform_types0.subcommand(mcmd);
        }
        let mut postal_codes0 = SubCommand::with_name("postal_codes")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets one postal code by ID.");
            postal_codes0 = postal_codes0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves a list of postal codes.");
            postal_codes0 = postal_codes0.subcommand(mcmd);
        }
        let mut projects0 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets one project by ID.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Retrieves a list of projects, possibly filtered. This method supports paging.",
            );
            projects0 = projects0.subcommand(mcmd);
        }
        let mut regions0 = SubCommand::with_name("regions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves a list of regions.");
            regions0 = regions0.subcommand(mcmd);
        }
        let mut remarketing_list_shares0 = SubCommand::with_name("remarketing_list_shares")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, patch and update");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets one remarketing list share by remarketing list ID.");
            remarketing_list_shares0 = remarketing_list_shares0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about(
                "Updates an existing remarketing list share. This method supports patch semantics.",
            );
            remarketing_list_shares0 = remarketing_list_shares0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update")
                .about("Updates an existing remarketing list share.");
            remarketing_list_shares0 = remarketing_list_shares0.subcommand(mcmd);
        }
        let mut remarketing_lists0 = SubCommand::with_name("remarketing_lists")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("get").about("Gets one remarketing list by ID.");
            remarketing_lists0 = remarketing_lists0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Inserts a new remarketing list.");
            remarketing_lists0 = remarketing_lists0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves a list of remarketing lists, possibly filtered. This method supports paging.");
            remarketing_lists0 = remarketing_lists0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about(
                "Updates an existing remarketing list. This method supports patch semantics.",
            );
            remarketing_lists0 = remarketing_lists0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("update").about("Updates an existing remarketing list.");
            remarketing_lists0 = remarketing_lists0.subcommand(mcmd);
        }
        let mut reports0 = SubCommand::with_name("reports")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, patch, run and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a report by its ID.");
            reports0 = reports0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves a report by its ID.");
            reports0 = reports0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a report.");
            reports0 = reports0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves list of reports.");
            reports0 = reports0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates a report. This method supports patch semantics.");
            reports0 = reports0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("run").about("Runs a report.");
            reports0 = reports0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a report.");
            reports0 = reports0.subcommand(mcmd);
        }
        let mut sites0 = SubCommand::with_name("sites")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("get").about("Gets one site by ID.");
            sites0 = sites0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Inserts a new site.");
            sites0 = sites0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Retrieves a list of sites, possibly filtered. This method supports paging.",
            );
            sites0 = sites0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates an existing site. This method supports patch semantics.");
            sites0 = sites0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates an existing site.");
            sites0 = sites0.subcommand(mcmd);
        }
        let mut sizes0 = SubCommand::with_name("sizes")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, insert and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets one size by ID.");
            sizes0 = sizes0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Inserts a new size.");
            sizes0 = sizes0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves a list of sizes, possibly filtered. Retrieved sizes are globally unique and may include values not currently in use by your account. Due to this, the list of sizes returned by this method may differ from the list seen in the Trafficking UI.");
            sizes0 = sizes0.subcommand(mcmd);
        }
        let mut subaccounts0 = SubCommand::with_name("subaccounts")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("get").about("Gets one subaccount by ID.");
            subaccounts0 = subaccounts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Inserts a new subaccount.");
            subaccounts0 = subaccounts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Gets a list of subaccounts, possibly filtered. This method supports paging.",
            );
            subaccounts0 = subaccounts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates an existing subaccount. This method supports patch semantics.");
            subaccounts0 = subaccounts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates an existing subaccount.");
            subaccounts0 = subaccounts0.subcommand(mcmd);
        }
        let mut targetable_remarketing_lists0 =
            SubCommand::with_name("targetable_remarketing_lists")
                .setting(AppSettings::ColoredHelp)
                .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets one remarketing list by ID.");
            targetable_remarketing_lists0 = targetable_remarketing_lists0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves a list of targetable remarketing lists, possibly filtered. This method supports paging.");
            targetable_remarketing_lists0 = targetable_remarketing_lists0.subcommand(mcmd);
        }
        let mut targeting_templates0 = SubCommand::with_name("targeting_templates")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("get").about("Gets one targeting template by ID.");
            targeting_templates0 = targeting_templates0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Inserts a new targeting template.");
            targeting_templates0 = targeting_templates0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves a list of targeting templates, optionally filtered. This method supports paging.");
            targeting_templates0 = targeting_templates0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about(
                "Updates an existing targeting template. This method supports patch semantics.",
            );
            targeting_templates0 = targeting_templates0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("update").about("Updates an existing targeting template.");
            targeting_templates0 = targeting_templates0.subcommand(mcmd);
        }
        let mut user_profiles0 = SubCommand::with_name("user_profiles")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets one user profile by ID.");
            user_profiles0 = user_profiles0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Retrieves list of user profiles for a user.");
            user_profiles0 = user_profiles0.subcommand(mcmd);
        }
        let mut user_role_permission_groups0 = SubCommand::with_name("user_role_permission_groups")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets one user role permission group by ID.");
            user_role_permission_groups0 = user_role_permission_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Gets a list of all supported user role permission groups.");
            user_role_permission_groups0 = user_role_permission_groups0.subcommand(mcmd);
        }
        let mut user_role_permissions0 = SubCommand::with_name("user_role_permissions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets one user role permission by ID.");
            user_role_permissions0 = user_role_permissions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Gets a list of user role permissions, possibly filtered.");
            user_role_permissions0 = user_role_permissions0.subcommand(mcmd);
        }
        let mut user_roles0 = SubCommand::with_name("user_roles")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an existing user role.");
            user_roles0 = user_roles0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets one user role by ID.");
            user_roles0 = user_roles0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Inserts a new user role.");
            user_roles0 = user_roles0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Retrieves a list of user roles, possibly filtered. This method supports paging.",
            );
            user_roles0 = user_roles0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates an existing user role. This method supports patch semantics.");
            user_roles0 = user_roles0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates an existing user role.");
            user_roles0 = user_roles0.subcommand(mcmd);
        }
        let mut video_formats0 = SubCommand::with_name("video_formats")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets one video format by ID.");
            video_formats0 = video_formats0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists available video formats.");
            video_formats0 = video_formats0.subcommand(mcmd);
        }
        let mut compatible_fields1 = SubCommand::with_name("compatible_fields")
            .setting(AppSettings::ColoredHelp)
            .about("methods: query");
        {
            let mcmd = SubCommand::with_name("query").about("Returns the fields that are compatible to be selected in the respective sections of a report criteria, given the fields already selected in the input report and user permissions.");
            compatible_fields1 = compatible_fields1.subcommand(mcmd);
        }
        let mut files1 = SubCommand::with_name("files")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Retrieves a report file. This method supports media download.");
            files1 = files1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists files for a report.");
            files1 = files1.subcommand(mcmd);
        }
        reports0 = reports0.subcommand(files1);
        reports0 = reports0.subcommand(compatible_fields1);
        app = app.subcommand(video_formats0);
        app = app.subcommand(user_roles0);
        app = app.subcommand(user_role_permissions0);
        app = app.subcommand(user_role_permission_groups0);
        app = app.subcommand(user_profiles0);
        app = app.subcommand(targeting_templates0);
        app = app.subcommand(targetable_remarketing_lists0);
        app = app.subcommand(subaccounts0);
        app = app.subcommand(sizes0);
        app = app.subcommand(sites0);
        app = app.subcommand(reports0);
        app = app.subcommand(remarketing_lists0);
        app = app.subcommand(remarketing_list_shares0);
        app = app.subcommand(regions0);
        app = app.subcommand(projects0);
        app = app.subcommand(postal_codes0);
        app = app.subcommand(platform_types0);
        app = app.subcommand(placements0);
        app = app.subcommand(placement_strategies0);
        app = app.subcommand(placement_groups0);
        app = app.subcommand(orders0);
        app = app.subcommand(order_documents0);
        app = app.subcommand(operating_systems0);
        app = app.subcommand(operating_system_versions0);
        app = app.subcommand(mobile_carriers0);
        app = app.subcommand(mobile_apps0);
        app = app.subcommand(metros0);
        app = app.subcommand(languages0);
        app = app.subcommand(inventory_items0);
        app = app.subcommand(floodlight_configurations0);
        app = app.subcommand(floodlight_activity_groups0);
        app = app.subcommand(floodlight_activities0);
        app = app.subcommand(files0);
        app = app.subcommand(event_tags0);
        app = app.subcommand(dynamic_targeting_keys0);
        app = app.subcommand(directory_sites0);
        app = app.subcommand(dimension_values0);
        app = app.subcommand(creatives0);
        app = app.subcommand(creative_groups0);
        app = app.subcommand(creative_fields0);
        app = app.subcommand(creative_field_values0);
        app = app.subcommand(creative_assets0);
        app = app.subcommand(countries0);
        app = app.subcommand(conversions0);
        app = app.subcommand(content_categories0);
        app = app.subcommand(connection_types0);
        app = app.subcommand(cities0);
        app = app.subcommand(change_logs0);
        app = app.subcommand(campaigns0);
        app = app.subcommand(campaign_creative_associations0);
        app = app.subcommand(browsers0);
        app = app.subcommand(advertisers0);
        app = app.subcommand(advertiser_landing_pages0);
        app = app.subcommand(advertiser_groups0);
        app = app.subcommand(ads0);
        app = app.subcommand(accounts0);
        app = app.subcommand(account_user_profiles0);
        app = app.subcommand(account_permissions0);
        app = app.subcommand(account_permission_groups0);
        app = app.subcommand(account_active_ad_summaries0);

        Self { app }
    }
}
use google_dfareporting3d1 as api;

fn main() {
    // TODO: set homedir afterwards, once the address is unmovable, or use Pin for the very first time
    // to allow a self-referential structure :D!
    let _home_dir = dirs::config_dir()
        .expect("configuration directory can be obtained")
        .join("google-service-cli");
    let outer = Outer::default_boxed();
    let app = outer.inner.app;
    let _matches = app.get_matches();
}
