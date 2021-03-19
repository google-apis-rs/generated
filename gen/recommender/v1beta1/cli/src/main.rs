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
        let mut app = App::new("recommender1_beta1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20210314")
            .about("")
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
        let mut billing_accounts0 = SubCommand::with_name("billing_accounts")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: locations");
        let mut folders0 = SubCommand::with_name("folders")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: locations");
        let mut organizations0 = SubCommand::with_name("organizations")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: locations");
        let mut projects0 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: locations");
        let mut locations1 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: insight_types and recommenders");
        let mut locations1 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: insight_types and recommenders");
        let mut locations1 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: insight_types and recommenders");
        let mut locations1 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: insight_types and recommenders");
        let mut insight_types2 = SubCommand::with_name("insight_types")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: insights");
        let mut recommenders2 = SubCommand::with_name("recommenders")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: recommendations");
        let mut insight_types2 = SubCommand::with_name("insight_types")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: insights");
        let mut recommenders2 = SubCommand::with_name("recommenders")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: recommendations");
        let mut insight_types2 = SubCommand::with_name("insight_types")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: insights");
        let mut recommenders2 = SubCommand::with_name("recommenders")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: recommendations");
        let mut insight_types2 = SubCommand::with_name("insight_types")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: insights");
        let mut recommenders2 = SubCommand::with_name("recommenders")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: recommendations");
        let mut insights3 = SubCommand::with_name("insights")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, list and mark_accepted");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the requested insight. Requires the recommender.*.get IAM permission for the specified insight type.");
            insights3 = insights3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists insights for a Cloud project. Requires the recommender.*.list IAM permission for the specified insight type.");
            insights3 = insights3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("mark_accepted").about("Marks the Insight State as Accepted. Users can use this method to indicate to the Recommender API that they have applied some action based on the insight. This stops the insight content from being updated. MarkInsightAccepted can be applied to insights in ACTIVE state. Requires the recommender.*.update IAM permission for the specified insight.");
            insights3 = insights3.subcommand(mcmd);
        }
        let mut recommendations3 = SubCommand::with_name("recommendations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, list, mark_claimed, mark_failed and mark_succeeded");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the requested recommendation. Requires the recommender.*.get IAM permission for the specified recommender.");
            recommendations3 = recommendations3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists recommendations for a Cloud project. Requires the recommender.*.list IAM permission for the specified recommender.");
            recommendations3 = recommendations3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("mark_claimed").about("Marks the Recommendation State as Claimed. Users can use this method to indicate to the Recommender API that they are starting to apply the recommendation themselves. This stops the recommendation content from being updated. Associated insights are frozen and placed in the ACCEPTED state. MarkRecommendationClaimed can be applied to recommendations in CLAIMED or ACTIVE state. Requires the recommender.*.update IAM permission for the specified recommender.");
            recommendations3 = recommendations3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("mark_failed").about("Marks the Recommendation State as Failed. Users can use this method to indicate to the Recommender API that they have applied the recommendation themselves, and the operation failed. This stops the recommendation content from being updated. Associated insights are frozen and placed in the ACCEPTED state. MarkRecommendationFailed can be applied to recommendations in ACTIVE, CLAIMED, SUCCEEDED, or FAILED state. Requires the recommender.*.update IAM permission for the specified recommender.");
            recommendations3 = recommendations3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("mark_succeeded").about("Marks the Recommendation State as Succeeded. Users can use this method to indicate to the Recommender API that they have applied the recommendation themselves, and the operation was successful. This stops the recommendation content from being updated. Associated insights are frozen and placed in the ACCEPTED state. MarkRecommendationSucceeded can be applied to recommendations in ACTIVE, CLAIMED, SUCCEEDED, or FAILED state. Requires the recommender.*.update IAM permission for the specified recommender.");
            recommendations3 = recommendations3.subcommand(mcmd);
        }
        let mut insights3 = SubCommand::with_name("insights")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, list and mark_accepted");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the requested insight. Requires the recommender.*.get IAM permission for the specified insight type.");
            insights3 = insights3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists insights for a Cloud project. Requires the recommender.*.list IAM permission for the specified insight type.");
            insights3 = insights3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("mark_accepted").about("Marks the Insight State as Accepted. Users can use this method to indicate to the Recommender API that they have applied some action based on the insight. This stops the insight content from being updated. MarkInsightAccepted can be applied to insights in ACTIVE state. Requires the recommender.*.update IAM permission for the specified insight.");
            insights3 = insights3.subcommand(mcmd);
        }
        let mut recommendations3 = SubCommand::with_name("recommendations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, list, mark_claimed, mark_failed and mark_succeeded");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the requested recommendation. Requires the recommender.*.get IAM permission for the specified recommender.");
            recommendations3 = recommendations3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists recommendations for a Cloud project. Requires the recommender.*.list IAM permission for the specified recommender.");
            recommendations3 = recommendations3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("mark_claimed").about("Marks the Recommendation State as Claimed. Users can use this method to indicate to the Recommender API that they are starting to apply the recommendation themselves. This stops the recommendation content from being updated. Associated insights are frozen and placed in the ACCEPTED state. MarkRecommendationClaimed can be applied to recommendations in CLAIMED or ACTIVE state. Requires the recommender.*.update IAM permission for the specified recommender.");
            recommendations3 = recommendations3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("mark_failed").about("Marks the Recommendation State as Failed. Users can use this method to indicate to the Recommender API that they have applied the recommendation themselves, and the operation failed. This stops the recommendation content from being updated. Associated insights are frozen and placed in the ACCEPTED state. MarkRecommendationFailed can be applied to recommendations in ACTIVE, CLAIMED, SUCCEEDED, or FAILED state. Requires the recommender.*.update IAM permission for the specified recommender.");
            recommendations3 = recommendations3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("mark_succeeded").about("Marks the Recommendation State as Succeeded. Users can use this method to indicate to the Recommender API that they have applied the recommendation themselves, and the operation was successful. This stops the recommendation content from being updated. Associated insights are frozen and placed in the ACCEPTED state. MarkRecommendationSucceeded can be applied to recommendations in ACTIVE, CLAIMED, SUCCEEDED, or FAILED state. Requires the recommender.*.update IAM permission for the specified recommender.");
            recommendations3 = recommendations3.subcommand(mcmd);
        }
        let mut insights3 = SubCommand::with_name("insights")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, list and mark_accepted");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the requested insight. Requires the recommender.*.get IAM permission for the specified insight type.");
            insights3 = insights3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists insights for a Cloud project. Requires the recommender.*.list IAM permission for the specified insight type.");
            insights3 = insights3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("mark_accepted").about("Marks the Insight State as Accepted. Users can use this method to indicate to the Recommender API that they have applied some action based on the insight. This stops the insight content from being updated. MarkInsightAccepted can be applied to insights in ACTIVE state. Requires the recommender.*.update IAM permission for the specified insight.");
            insights3 = insights3.subcommand(mcmd);
        }
        let mut recommendations3 = SubCommand::with_name("recommendations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, list, mark_claimed, mark_failed and mark_succeeded");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the requested recommendation. Requires the recommender.*.get IAM permission for the specified recommender.");
            recommendations3 = recommendations3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists recommendations for a Cloud project. Requires the recommender.*.list IAM permission for the specified recommender.");
            recommendations3 = recommendations3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("mark_claimed").about("Marks the Recommendation State as Claimed. Users can use this method to indicate to the Recommender API that they are starting to apply the recommendation themselves. This stops the recommendation content from being updated. Associated insights are frozen and placed in the ACCEPTED state. MarkRecommendationClaimed can be applied to recommendations in CLAIMED or ACTIVE state. Requires the recommender.*.update IAM permission for the specified recommender.");
            recommendations3 = recommendations3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("mark_failed").about("Marks the Recommendation State as Failed. Users can use this method to indicate to the Recommender API that they have applied the recommendation themselves, and the operation failed. This stops the recommendation content from being updated. Associated insights are frozen and placed in the ACCEPTED state. MarkRecommendationFailed can be applied to recommendations in ACTIVE, CLAIMED, SUCCEEDED, or FAILED state. Requires the recommender.*.update IAM permission for the specified recommender.");
            recommendations3 = recommendations3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("mark_succeeded").about("Marks the Recommendation State as Succeeded. Users can use this method to indicate to the Recommender API that they have applied the recommendation themselves, and the operation was successful. This stops the recommendation content from being updated. Associated insights are frozen and placed in the ACCEPTED state. MarkRecommendationSucceeded can be applied to recommendations in ACTIVE, CLAIMED, SUCCEEDED, or FAILED state. Requires the recommender.*.update IAM permission for the specified recommender.");
            recommendations3 = recommendations3.subcommand(mcmd);
        }
        let mut insights3 = SubCommand::with_name("insights")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, list and mark_accepted");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the requested insight. Requires the recommender.*.get IAM permission for the specified insight type.");
            insights3 = insights3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists insights for a Cloud project. Requires the recommender.*.list IAM permission for the specified insight type.");
            insights3 = insights3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("mark_accepted").about("Marks the Insight State as Accepted. Users can use this method to indicate to the Recommender API that they have applied some action based on the insight. This stops the insight content from being updated. MarkInsightAccepted can be applied to insights in ACTIVE state. Requires the recommender.*.update IAM permission for the specified insight.");
            insights3 = insights3.subcommand(mcmd);
        }
        let mut recommendations3 = SubCommand::with_name("recommendations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, list, mark_claimed, mark_failed and mark_succeeded");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the requested recommendation. Requires the recommender.*.get IAM permission for the specified recommender.");
            recommendations3 = recommendations3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists recommendations for a Cloud project. Requires the recommender.*.list IAM permission for the specified recommender.");
            recommendations3 = recommendations3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("mark_claimed").about("Marks the Recommendation State as Claimed. Users can use this method to indicate to the Recommender API that they are starting to apply the recommendation themselves. This stops the recommendation content from being updated. Associated insights are frozen and placed in the ACCEPTED state. MarkRecommendationClaimed can be applied to recommendations in CLAIMED or ACTIVE state. Requires the recommender.*.update IAM permission for the specified recommender.");
            recommendations3 = recommendations3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("mark_failed").about("Marks the Recommendation State as Failed. Users can use this method to indicate to the Recommender API that they have applied the recommendation themselves, and the operation failed. This stops the recommendation content from being updated. Associated insights are frozen and placed in the ACCEPTED state. MarkRecommendationFailed can be applied to recommendations in ACTIVE, CLAIMED, SUCCEEDED, or FAILED state. Requires the recommender.*.update IAM permission for the specified recommender.");
            recommendations3 = recommendations3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("mark_succeeded").about("Marks the Recommendation State as Succeeded. Users can use this method to indicate to the Recommender API that they have applied the recommendation themselves, and the operation was successful. This stops the recommendation content from being updated. Associated insights are frozen and placed in the ACCEPTED state. MarkRecommendationSucceeded can be applied to recommendations in ACTIVE, CLAIMED, SUCCEEDED, or FAILED state. Requires the recommender.*.update IAM permission for the specified recommender.");
            recommendations3 = recommendations3.subcommand(mcmd);
        }
        recommenders2 = recommenders2.subcommand(recommendations3);
        insight_types2 = insight_types2.subcommand(insights3);
        recommenders2 = recommenders2.subcommand(recommendations3);
        insight_types2 = insight_types2.subcommand(insights3);
        recommenders2 = recommenders2.subcommand(recommendations3);
        insight_types2 = insight_types2.subcommand(insights3);
        recommenders2 = recommenders2.subcommand(recommendations3);
        insight_types2 = insight_types2.subcommand(insights3);
        locations1 = locations1.subcommand(recommenders2);
        locations1 = locations1.subcommand(insight_types2);
        locations1 = locations1.subcommand(recommenders2);
        locations1 = locations1.subcommand(insight_types2);
        locations1 = locations1.subcommand(recommenders2);
        locations1 = locations1.subcommand(insight_types2);
        locations1 = locations1.subcommand(recommenders2);
        locations1 = locations1.subcommand(insight_types2);
        projects0 = projects0.subcommand(locations1);
        organizations0 = organizations0.subcommand(locations1);
        folders0 = folders0.subcommand(locations1);
        billing_accounts0 = billing_accounts0.subcommand(locations1);
        app = app.subcommand(projects0);
        app = app.subcommand(organizations0);
        app = app.subcommand(folders0);
        app = app.subcommand(billing_accounts0);

        Self { app }
    }
}
use google_recommender1_beta1 as api;

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
