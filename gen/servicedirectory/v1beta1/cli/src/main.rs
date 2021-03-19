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
        let mut app = App::new("servicedirectory1_beta1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20210303")
            .about("Service Directory is a platform for discovering, publishing, and connecting services. ")
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
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets information about a location.");
            locations1 = locations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists information about the supported locations for this service.");
            locations1 = locations1.subcommand(mcmd);
        }
        let mut namespaces2 = SubCommand::with_name("namespaces")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_iam_policy, list, patch, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a namespace, and returns the new namespace.");
            namespaces2 = namespaces2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a namespace. This also deletes all services and endpoints in the namespace.");
            namespaces2 = namespaces2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a namespace.");
            namespaces2 = namespaces2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy")
                .about("Gets the IAM Policy for a resource (namespace or service only).");
            namespaces2 = namespaces2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all namespaces.");
            namespaces2 = namespaces2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a namespace.");
            namespaces2 = namespaces2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy")
                .about("Sets the IAM Policy for a resource (namespace or service only).");
            namespaces2 = namespaces2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions")
                .about("Tests IAM permissions for a resource (namespace or service only).");
            namespaces2 = namespaces2.subcommand(mcmd);
        }
        let mut services3 = SubCommand::with_name("services")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_iam_policy, list, patch, resolve, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a service, and returns the new service.");
            services3 = services3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about(
                "Deletes a service. This also deletes all endpoints associated with the service.",
            );
            services3 = services3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a service.");
            services3 = services3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy")
                .about("Gets the IAM Policy for a resource (namespace or service only).");
            services3 = services3.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists all services belonging to a namespace.");
            services3 = services3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a service.");
            services3 = services3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("resolve").about("Returns a service and its associated endpoints. Resolving a service is not considered an active developer method.");
            services3 = services3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy")
                .about("Sets the IAM Policy for a resource (namespace or service only).");
            services3 = services3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions")
                .about("Tests IAM permissions for a resource (namespace or service only).");
            services3 = services3.subcommand(mcmd);
        }
        let mut endpoints4 = SubCommand::with_name("endpoints")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates an endpoint, and returns the new endpoint.");
            endpoints4 = endpoints4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an endpoint.");
            endpoints4 = endpoints4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets an endpoint.");
            endpoints4 = endpoints4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all endpoints.");
            endpoints4 = endpoints4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an endpoint.");
            endpoints4 = endpoints4.subcommand(mcmd);
        }
        services3 = services3.subcommand(endpoints4);
        namespaces2 = namespaces2.subcommand(services3);
        locations1 = locations1.subcommand(namespaces2);
        projects0 = projects0.subcommand(locations1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_servicedirectory1_beta1 as api;

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
