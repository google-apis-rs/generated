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
        let mut app = App::new("retail2")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20210312")
            .about("Cloud Retail service enables customers to build end-to-end personalized recommendation systems without requiring a high level of expertise in machine learning, recommendation system, or Google Cloud.")
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
        let mut projects0 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: locations");
        let mut locations1 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: catalogs and operations");
        let mut catalogs2 = SubCommand::with_name("catalogs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list and patch");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all the Catalogs associated with the project.");
            catalogs2 = catalogs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the Catalogs.");
            catalogs2 = catalogs2.subcommand(mcmd);
        }
        let mut operations2 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations2 = operations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request. If the server doesn\'t support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `\"/v1/{name=users/*}/operations\"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.");
            operations2 = operations2.subcommand(mcmd);
        }
        let mut branches3 = SubCommand::with_name("branches")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: operations and products");
        let mut operations3 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations3 = operations3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request. If the server doesn\'t support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `\"/v1/{name=users/*}/operations\"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.");
            operations3 = operations3.subcommand(mcmd);
        }
        let mut placements3 = SubCommand::with_name("placements")
            .setting(AppSettings::ColoredHelp)
            .about("methods: predict");
        {
            let mcmd = SubCommand::with_name("predict").about("Makes a recommendation prediction.");
            placements3 = placements3.subcommand(mcmd);
        }
        let mut user_events3 = SubCommand::with_name("user_events")
            .setting(AppSettings::ColoredHelp)
            .about("methods: collect, import, purge, rejoin and write");
        {
            let mcmd = SubCommand::with_name("collect").about("Writes a single user event from the browser. This uses a GET request to due to browser restriction of POST-ing to a 3rd party domain. This method is used only by the Retail API JavaScript pixel and Google Tag Manager. Users should not call this method directly.");
            user_events3 = user_events3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("import").about("Bulk import of User events. Request processing might be synchronous. Events that already exist are skipped. Use this method for backfilling historical user events. Operation.response is of type ImportResponse. Note that it is possible for a subset of the items to be successfully inserted. Operation.metadata is of type ImportMetadata.");
            user_events3 = user_events3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("purge").about("Deletes permanently all user events specified by the filter provided. Depending on the number of events specified by the filter, this operation could take hours or days to complete. To test a filter, use the list command first.");
            user_events3 = user_events3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("rejoin").about("Triggers a user event rejoin operation with latest product catalog. Events will not be annotated with detailed product information if product is missing from the catalog at the time the user event is ingested, and these events are stored as unjoined events with a limited usage on training and serving. This API can be used to trigger a \'join\' operation on specified events with latest version of product catalog. It can also be used to correct events joined with wrong product catalog.");
            user_events3 = user_events3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("write").about("Writes a single user event.");
            user_events3 = user_events3.subcommand(mcmd);
        }
        let mut operations4 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations4 = operations4.subcommand(mcmd);
        }
        let mut products4 = SubCommand::with_name("products")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, import and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a Product.");
            products4 = products4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a Product.");
            products4 = products4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a Product.");
            products4 = products4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("import").about("Bulk import of multiple Products. Request processing may be synchronous. No partial updating is supported. Non-existing items are created. Note that it is possible for a subset of the Products to be successfully updated.");
            products4 = products4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a Product.");
            products4 = products4.subcommand(mcmd);
        }
        branches3 = branches3.subcommand(products4);
        branches3 = branches3.subcommand(operations4);
        catalogs2 = catalogs2.subcommand(user_events3);
        catalogs2 = catalogs2.subcommand(placements3);
        catalogs2 = catalogs2.subcommand(operations3);
        catalogs2 = catalogs2.subcommand(branches3);
        locations1 = locations1.subcommand(operations2);
        locations1 = locations1.subcommand(catalogs2);
        projects0 = projects0.subcommand(locations1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_retail2 as api;

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
