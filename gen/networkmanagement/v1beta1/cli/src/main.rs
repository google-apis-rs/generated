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
        let mut app = App::new("networkmanagement1_beta1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200520")
            .about("The Network Management API provides a collection of network performance monitoring and diagnostic capabilities.")
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
        let mut global2 = SubCommand::with_name("global")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: connectivity_tests and operations");
        let mut connectivity_tests3 = SubCommand::with_name("connectivity_tests")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_iam_policy, list, patch, rerun, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new Connectivity Test.\nAfter you create a test, the reachability analysis is performed as part\nof the long running operation, which completes when the analysis completes.\n\nIf the endpoint specifications in `ConnectivityTest` are invalid\n(for example, containing non-existent resources in the network, or you\ndon\'t have read permissions to the network configurations of listed\nprojects), then the reachability result returns a value of `UNKNOWN`.\n\nIf the endpoint specifications in `ConnectivityTest` are\nincomplete, the reachability result returns a value of\n<code>AMBIGUOUS</code>. For more information,\nsee the Connectivity Test documentation.");
            connectivity_tests3 = connectivity_tests3.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes a specific `ConnectivityTest`.");
            connectivity_tests3 = connectivity_tests3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets the details of a specific Connectivity Test.");
            connectivity_tests3 = connectivity_tests3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource.\nReturns an empty policy if the resource exists and does not have a policy\nset.");
            connectivity_tests3 = connectivity_tests3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all Connectivity Tests owned by a project.");
            connectivity_tests3 = connectivity_tests3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the configuration of an existing `ConnectivityTest`.\nAfter you update a test, the reachability analysis is performed as part\nof the long running operation, which completes when the analysis completes.\nThe Reachability state in the test resource is updated with the new result.\n\nIf the endpoint specifications in `ConnectivityTest` are invalid\n(for example, they contain non-existent resources in the network, or the\nuser does not have read permissions to the network configurations of\nlisted projects), then the reachability result returns a value of\n<code>UNKNOWN</code>.\n\nIf the endpoint specifications in `ConnectivityTest` are incomplete, the\nreachability result returns a value of `AMBIGUOUS`. See the documentation\nin `ConnectivityTest` for for more details.");
            connectivity_tests3 = connectivity_tests3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("rerun").about("Rerun an existing `ConnectivityTest`.\nAfter the user triggers the rerun, the reachability analysis is performed\nas part of the long running operation, which completes when the analysis\ncompletes.\n\nEven though the test configuration remains the same, the reachability\nresult may change due to underlying network configuration changes.\n\nIf the endpoint specifications in `ConnectivityTest` become invalid (for\nexample, specified resources are deleted in the network, or you lost\nread permissions to the network configurations of listed projects), then\nthe reachability result returns a value of `UNKNOWN`.");
            connectivity_tests3 = connectivity_tests3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any\nexisting policy.\n\nCan return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.");
            connectivity_tests3 = connectivity_tests3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource.\nIf the resource does not exist, this will return an empty set of\npermissions, not a `NOT_FOUND` error.\n\nNote: This operation is designed to be used for building permission-aware\nUIs and command-line tools, not for authorization checking. This operation\nmay \"fail open\" without warning.");
            connectivity_tests3 = connectivity_tests3.subcommand(mcmd);
        }
        let mut operations3 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, delete, get and list");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running operation.  The server\nmakes a best effort to cancel the operation, but success is not\nguaranteed.  If the server doesn\'t support this method, it returns\n`google.rpc.Code.UNIMPLEMENTED`.  Clients can use\nOperations.GetOperation or\nother methods to check whether the cancellation succeeded or whether the\noperation completed despite cancellation. On successful cancellation,\nthe operation is not deleted; instead, it becomes an operation with\nan Operation.error value with a google.rpc.Status.code of 1,\ncorresponding to `Code.CANCELLED`.");
            operations3 = operations3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a long-running operation. This method indicates that the client is\nno longer interested in the operation result. It does not cancel the\noperation. If the server doesn\'t support this method, it returns\n`google.rpc.Code.UNIMPLEMENTED`.");
            operations3 = operations3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation.  Clients can use this\nmethod to poll the operation result at intervals as recommended by the API\nservice.");
            operations3 = operations3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request. If the\nserver doesn\'t support this method, it returns `UNIMPLEMENTED`.\n\nNOTE: the `name` binding allows API services to override the binding\nto use different resource name schemes, such as `users/*/operations`. To\noverride the binding, API services can add a binding such as\n`\"/v1/{name=users/*}/operations\"` to their service configuration.\nFor backwards compatibility, the default name includes the operations\ncollection id, however overriding users must ensure the name binding\nis the parent resource, without the operations collection id.");
            operations3 = operations3.subcommand(mcmd);
        }
        global2 = global2.subcommand(operations3);
        global2 = global2.subcommand(connectivity_tests3);
        locations1 = locations1.subcommand(global2);
        projects0 = projects0.subcommand(locations1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_networkmanagement1_beta1 as api;

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
