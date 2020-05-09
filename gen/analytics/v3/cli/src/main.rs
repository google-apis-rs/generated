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
        let mut app = App::new("analytics3")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20190807")
            .about("Views and manages your Google Analytics data.")
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
        let mut data0 = SubCommand::with_name("data")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: ga, mcf and realtime");
        let mut management0 = SubCommand::with_name("management")
                        .setting(AppSettings::ColoredHelp)
                        .about("sub-resources: account_summaries, account_user_links, accounts, client_id, custom_data_sources, custom_dimensions, custom_metrics, experiments, filters, goals, profile_filter_links, profile_user_links, profiles, remarketing_audience, segments, unsampled_reports, uploads, web_property_ad_words_links, webproperties and webproperty_user_links");
        let mut metadata0 = SubCommand::with_name("metadata")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: columns");
        let mut provisioning0 = SubCommand::with_name("provisioning")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create_account_ticket and create_account_tree");
        {
            let mcmd =
                SubCommand::with_name("create_account_ticket").about("Creates an account ticket.");
            provisioning0 = provisioning0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create_account_tree").about("Provision account.");
            provisioning0 = provisioning0.subcommand(mcmd);
        }
        let mut user_deletion0 = SubCommand::with_name("user_deletion")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: user_deletion_request");
        let mut ga1 = SubCommand::with_name("ga")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd =
                SubCommand::with_name("get").about("Returns Analytics data for a view (profile).");
            ga1 = ga1.subcommand(mcmd);
        }
        let mut mcf1 = SubCommand::with_name("mcf")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Returns Analytics Multi-Channel Funnels data for a view (profile).");
            mcf1 = mcf1.subcommand(mcmd);
        }
        let mut realtime1 = SubCommand::with_name("realtime")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd =
                SubCommand::with_name("get").about("Returns real time data for a view (profile).");
            realtime1 = realtime1.subcommand(mcmd);
        }
        let mut account_summaries1 = SubCommand::with_name("account_summaries")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists account summaries (lightweight tree comprised of accounts/properties/profiles) to which the user has access.");
            account_summaries1 = account_summaries1.subcommand(mcmd);
        }
        let mut account_user_links1 = SubCommand::with_name("account_user_links")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, insert, list and update");
        {
            let mcmd =
                SubCommand::with_name("delete").about("Removes a user from the given account.");
            account_user_links1 = account_user_links1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("insert").about("Adds a new user to the given account.");
            account_user_links1 = account_user_links1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists account-user links for a given account.");
            account_user_links1 = account_user_links1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update")
                .about("Updates permissions for an existing user on the given account.");
            account_user_links1 = account_user_links1.subcommand(mcmd);
        }
        let mut accounts1 = SubCommand::with_name("accounts")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all accounts to which the user has access.");
            accounts1 = accounts1.subcommand(mcmd);
        }
        let mut client_id1 = SubCommand::with_name("client_id")
            .setting(AppSettings::ColoredHelp)
            .about("methods: hash_client_id");
        {
            let mcmd = SubCommand::with_name("hash_client_id").about("Hashes the given Client ID.");
            client_id1 = client_id1.subcommand(mcmd);
        }
        let mut custom_data_sources1 = SubCommand::with_name("custom_data_sources")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("List custom data sources to which the user has access.");
            custom_data_sources1 = custom_data_sources1.subcommand(mcmd);
        }
        let mut custom_dimensions1 = SubCommand::with_name("custom_dimensions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Get a custom dimension to which the user has access.");
            custom_dimensions1 = custom_dimensions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Create a new custom dimension.");
            custom_dimensions1 = custom_dimensions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists custom dimensions to which the user has access.");
            custom_dimensions1 = custom_dimensions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about(
                "Updates an existing custom dimension. This method supports patch semantics.",
            );
            custom_dimensions1 = custom_dimensions1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("update").about("Updates an existing custom dimension.");
            custom_dimensions1 = custom_dimensions1.subcommand(mcmd);
        }
        let mut custom_metrics1 = SubCommand::with_name("custom_metrics")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Get a custom metric to which the user has access.");
            custom_metrics1 = custom_metrics1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Create a new custom metric.");
            custom_metrics1 = custom_metrics1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists custom metrics to which the user has access.");
            custom_metrics1 = custom_metrics1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates an existing custom metric. This method supports patch semantics.");
            custom_metrics1 = custom_metrics1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates an existing custom metric.");
            custom_metrics1 = custom_metrics1.subcommand(mcmd);
        }
        let mut experiments1 = SubCommand::with_name("experiments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Delete an experiment.");
            experiments1 = experiments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Returns an experiment to which the user has access.");
            experiments1 = experiments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Create a new experiment.");
            experiments1 = experiments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists experiments to which the user has access.");
            experiments1 = experiments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Update an existing experiment. This method supports patch semantics.");
            experiments1 = experiments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Update an existing experiment.");
            experiments1 = experiments1.subcommand(mcmd);
        }
        let mut filters1 = SubCommand::with_name("filters")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Delete a filter.");
            filters1 = filters1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Returns filters to which the user has access.");
            filters1 = filters1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Create a new filter.");
            filters1 = filters1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all filters for an account");
            filters1 = filters1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates an existing filter. This method supports patch semantics.");
            filters1 = filters1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates an existing filter.");
            filters1 = filters1.subcommand(mcmd);
        }
        let mut goals1 = SubCommand::with_name("goals")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, insert, list, patch and update");
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets a goal to which the user has access.");
            goals1 = goals1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Create a new goal.");
            goals1 = goals1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists goals to which the user has access.");
            goals1 = goals1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates an existing goal. This method supports patch semantics.");
            goals1 = goals1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates an existing goal.");
            goals1 = goals1.subcommand(mcmd);
        }
        let mut profile_filter_links1 = SubCommand::with_name("profile_filter_links")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Delete a profile filter link.");
            profile_filter_links1 = profile_filter_links1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns a single profile filter link.");
            profile_filter_links1 = profile_filter_links1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Create a new profile filter link.");
            profile_filter_links1 = profile_filter_links1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all profile filter links for a profile.");
            profile_filter_links1 = profile_filter_links1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about(
                "Update an existing profile filter link. This method supports patch semantics.",
            );
            profile_filter_links1 = profile_filter_links1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("update").about("Update an existing profile filter link.");
            profile_filter_links1 = profile_filter_links1.subcommand(mcmd);
        }
        let mut profile_user_links1 = SubCommand::with_name("profile_user_links")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, insert, list and update");
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Removes a user from the given view (profile).");
            profile_user_links1 = profile_user_links1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert")
                .about("Adds a new user to the given view (profile).");
            profile_user_links1 = profile_user_links1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists profile-user links for a given view (profile).");
            profile_user_links1 = profile_user_links1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update")
                .about("Updates permissions for an existing user on the given view (profile).");
            profile_user_links1 = profile_user_links1.subcommand(mcmd);
        }
        let mut profiles1 = SubCommand::with_name("profiles")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a view (profile).");
            profiles1 = profiles1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets a view (profile) to which the user has access.");
            profiles1 = profiles1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Create a new view (profile).");
            profiles1 = profiles1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists views (profiles) to which the user has access.");
            profiles1 = profiles1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates an existing view (profile). This method supports patch semantics.");
            profiles1 = profiles1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates an existing view (profile).");
            profiles1 = profiles1.subcommand(mcmd);
        }
        let mut remarketing_audience1 = SubCommand::with_name("remarketing_audience")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Delete a remarketing audience.");
            remarketing_audience1 = remarketing_audience1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets a remarketing audience to which the user has access.");
            remarketing_audience1 = remarketing_audience1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a new remarketing audience.");
            remarketing_audience1 = remarketing_audience1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists remarketing audiences to which the user has access.");
            remarketing_audience1 = remarketing_audience1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about(
                "Updates an existing remarketing audience. This method supports patch semantics.",
            );
            remarketing_audience1 = remarketing_audience1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("update").about("Updates an existing remarketing audience.");
            remarketing_audience1 = remarketing_audience1.subcommand(mcmd);
        }
        let mut segments1 = SubCommand::with_name("segments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists segments to which the user has access.");
            segments1 = segments1.subcommand(mcmd);
        }
        let mut unsampled_reports1 = SubCommand::with_name("unsampled_reports")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert and list");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an unsampled report.");
            unsampled_reports1 = unsampled_reports1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns a single unsampled report.");
            unsampled_reports1 = unsampled_reports1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Create a new unsampled report.");
            unsampled_reports1 = unsampled_reports1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists unsampled reports to which the user has access.");
            unsampled_reports1 = unsampled_reports1.subcommand(mcmd);
        }
        let mut uploads1 = SubCommand::with_name("uploads")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete_upload_data, get, list and upload_data");
        {
            let mcmd = SubCommand::with_name("delete_upload_data")
                .about("Delete data associated with a previous upload.");
            uploads1 = uploads1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("List uploads to which the user has access.");
            uploads1 = uploads1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("List uploads to which the user has access.");
            uploads1 = uploads1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("upload_data").about("Upload data for a custom data source.");
            uploads1 = uploads1.subcommand(mcmd);
        }
        let mut web_property_ad_words_links1 = SubCommand::with_name("web_property_ad_words_links")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, patch and update");
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes a web property-Google Ads link.");
            web_property_ad_words_links1 = web_property_ad_words_links1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Returns a web property-Google Ads link to which the user has access.");
            web_property_ad_words_links1 = web_property_ad_words_links1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("insert").about("Creates a webProperty-Google Ads link.");
            web_property_ad_words_links1 = web_property_ad_words_links1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists webProperty-Google Ads links for a given web property.");
            web_property_ad_words_links1 = web_property_ad_words_links1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an existing webProperty-Google Ads link. This method supports patch semantics.");
            web_property_ad_words_links1 = web_property_ad_words_links1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update")
                .about("Updates an existing webProperty-Google Ads link.");
            web_property_ad_words_links1 = web_property_ad_words_links1.subcommand(mcmd);
        }
        let mut webproperties1 = SubCommand::with_name("webproperties")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets a web property to which the user has access.");
            webproperties1 = webproperties1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Create a new property if the account has fewer than 20 properties. Web properties are visible in the Google Analytics interface only if they have at least one profile.");
            webproperties1 = webproperties1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists web properties to which the user has access.");
            webproperties1 = webproperties1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates an existing web property. This method supports patch semantics.");
            webproperties1 = webproperties1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates an existing web property.");
            webproperties1 = webproperties1.subcommand(mcmd);
        }
        let mut webproperty_user_links1 = SubCommand::with_name("webproperty_user_links")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, insert, list and update");
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Removes a user from the given web property.");
            webproperty_user_links1 = webproperty_user_links1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("insert").about("Adds a new user to the given web property.");
            webproperty_user_links1 = webproperty_user_links1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists webProperty-user links for a given web property.");
            webproperty_user_links1 = webproperty_user_links1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update")
                .about("Updates permissions for an existing user on the given web property.");
            webproperty_user_links1 = webproperty_user_links1.subcommand(mcmd);
        }
        let mut columns1 = SubCommand::with_name("columns")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists all columns for a report type");
            columns1 = columns1.subcommand(mcmd);
        }
        let mut user_deletion_request1 = SubCommand::with_name("user_deletion_request")
            .setting(AppSettings::ColoredHelp)
            .about("methods: upsert");
        {
            let mcmd =
                SubCommand::with_name("upsert").about("Insert or update a user deletion requests.");
            user_deletion_request1 = user_deletion_request1.subcommand(mcmd);
        }
        user_deletion0 = user_deletion0.subcommand(user_deletion_request1);
        metadata0 = metadata0.subcommand(columns1);
        management0 = management0.subcommand(webproperty_user_links1);
        management0 = management0.subcommand(webproperties1);
        management0 = management0.subcommand(web_property_ad_words_links1);
        management0 = management0.subcommand(uploads1);
        management0 = management0.subcommand(unsampled_reports1);
        management0 = management0.subcommand(segments1);
        management0 = management0.subcommand(remarketing_audience1);
        management0 = management0.subcommand(profiles1);
        management0 = management0.subcommand(profile_user_links1);
        management0 = management0.subcommand(profile_filter_links1);
        management0 = management0.subcommand(goals1);
        management0 = management0.subcommand(filters1);
        management0 = management0.subcommand(experiments1);
        management0 = management0.subcommand(custom_metrics1);
        management0 = management0.subcommand(custom_dimensions1);
        management0 = management0.subcommand(custom_data_sources1);
        management0 = management0.subcommand(client_id1);
        management0 = management0.subcommand(accounts1);
        management0 = management0.subcommand(account_user_links1);
        management0 = management0.subcommand(account_summaries1);
        data0 = data0.subcommand(realtime1);
        data0 = data0.subcommand(mcf1);
        data0 = data0.subcommand(ga1);
        app = app.subcommand(user_deletion0);
        app = app.subcommand(provisioning0);
        app = app.subcommand(metadata0);
        app = app.subcommand(management0);
        app = app.subcommand(data0);

        Self { app }
    }
}
use google_analytics3 as api;

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
