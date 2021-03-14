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
        let mut app = App::new("serviceusage1_beta1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20210310")
            .about("Enables services that service consumers want to use on Google Cloud Platform, lists the available or enabled services, or disables services that service consumers no longer use.")
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
        let mut operations0 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations0 = operations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request. If the server doesn\'t support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `\"/v1/{name=users/*}/operations\"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.");
            operations0 = operations0.subcommand(mcmd);
        }
        let mut services0 = SubCommand::with_name("services")
            .setting(AppSettings::ColoredHelp)
            .about(
                "methods: batch_enable, disable, enable, generate_service_identity, get and list",
            );
        {
            let mcmd = SubCommand::with_name("batch_enable").about("Enable multiple services on a project. The operation is atomic: if enabling any service fails, then the entire batch fails, and no state changes occur. Operation");
            services0 = services0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("disable").about("Disable a service so that it can no longer be used with a project. This prevents unintended usage that may cause unexpected billing charges or security leaks. It is not valid to call the disable method on a service that is not currently enabled. Callers will receive a `FAILED_PRECONDITION` status if the target service is not currently enabled. Operation");
            services0 = services0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("enable")
                .about("Enable a service so that it can be used with a project. Operation");
            services0 = services0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("generate_service_identity")
                .about("Generate service identity for service.");
            services0 = services0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Returns the service configuration and enabled state for a given service.");
            services0 = services0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List all services available to the specified project, and the current state of those services with respect to the project. The list includes all public services, all services for which the calling user has the `servicemanagement.services.bind` permission, and all services that have already been enabled on the project. The list can be filtered to only include services in a specific state, for example to only include services enabled on the project.");
            services0 = services0.subcommand(mcmd);
        }
        let mut consumer_quota_metrics1 = SubCommand::with_name("consumer_quota_metrics")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, import_admin_overrides, import_consumer_overrides and list");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Retrieves a summary of quota information for a specific quota metric");
            consumer_quota_metrics1 = consumer_quota_metrics1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("import_admin_overrides").about("Create or update multiple admin overrides atomically, all on the same consumer, but on many different metrics or limits. The name field in the quota override message should not be set.");
            consumer_quota_metrics1 = consumer_quota_metrics1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("import_consumer_overrides").about("Create or update multiple consumer overrides atomically, all on the same consumer, but on many different metrics or limits. The name field in the quota override message should not be set.");
            consumer_quota_metrics1 = consumer_quota_metrics1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves a summary of all quota information visible to the service consumer, organized by service metric. Each metric includes information about all of its defined limits. Each limit includes the limit configuration (quota unit, preciseness, default value), the current effective limit value, and all of the overrides applied to the limit.");
            consumer_quota_metrics1 = consumer_quota_metrics1.subcommand(mcmd);
        }
        let mut limits2 = SubCommand::with_name("limits")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Retrieves a summary of quota information for a specific quota limit.");
            limits2 = limits2.subcommand(mcmd);
        }
        let mut admin_overrides3 = SubCommand::with_name("admin_overrides")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates an admin override. An admin override is applied by an administrator of a parent folder or parent organization of the consumer receiving the override. An admin override is intended to limit the amount of quota the consumer can use out of the total quota pool allocated to all children of the folder or organization.");
            admin_overrides3 = admin_overrides3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an admin override.");
            admin_overrides3 = admin_overrides3.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists all admin overrides on this limit.");
            admin_overrides3 = admin_overrides3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an admin override.");
            admin_overrides3 = admin_overrides3.subcommand(mcmd);
        }
        let mut consumer_overrides3 = SubCommand::with_name("consumer_overrides")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a consumer override. A consumer override is applied to the consumer on its own authority to limit its own quota usage. Consumer overrides cannot be used to grant more quota than would be allowed by admin overrides, producer overrides, or the default limit of the service.");
            consumer_overrides3 = consumer_overrides3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a consumer override.");
            consumer_overrides3 = consumer_overrides3.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists all consumer overrides on this limit.");
            consumer_overrides3 = consumer_overrides3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a consumer override.");
            consumer_overrides3 = consumer_overrides3.subcommand(mcmd);
        }
        limits2 = limits2.subcommand(consumer_overrides3);
        limits2 = limits2.subcommand(admin_overrides3);
        consumer_quota_metrics1 = consumer_quota_metrics1.subcommand(limits2);
        services0 = services0.subcommand(consumer_quota_metrics1);
        app = app.subcommand(services0);
        app = app.subcommand(operations0);

        Self { app }
    }
}
use google_serviceusage1_beta1 as api;

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
