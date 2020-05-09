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
        let mut app = App::new("bigtableadmin2")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200424")
            .about("Administer your Cloud Bigtable tables and instances.")
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
            .about("methods: cancel, delete and get");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running operation.  The server\nmakes a best effort to cancel the operation, but success is not\nguaranteed.  If the server doesn\'t support this method, it returns\n`google.rpc.Code.UNIMPLEMENTED`.  Clients can use\nOperations.GetOperation or\nother methods to check whether the cancellation succeeded or whether the\noperation completed despite cancellation. On successful cancellation,\nthe operation is not deleted; instead, it becomes an operation with\nan Operation.error value with a google.rpc.Status.code of 1,\ncorresponding to `Code.CANCELLED`.");
            operations0 = operations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a long-running operation. This method indicates that the client is\nno longer interested in the operation result. It does not cancel the\noperation. If the server doesn\'t support this method, it returns\n`google.rpc.Code.UNIMPLEMENTED`.");
            operations0 = operations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation.  Clients can use this\nmethod to poll the operation result at intervals as recommended by the API\nservice.");
            operations0 = operations0.subcommand(mcmd);
        }
        let mut projects0 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: instances and locations");
        let mut projects1 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: operations");
        let mut instances1 = SubCommand::with_name("instances")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_iam_policy, list, partial_update_instance, set_iam_policy, test_iam_permissions and update");
        {
            let mcmd =
                SubCommand::with_name("create").about("Create an instance within a project.");
            instances1 = instances1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Delete an instance from a project.");
            instances1 = instances1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets information about an instance.");
            instances1 = instances1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for an instance resource. Returns an empty\npolicy if an instance exists but does not have a policy set.");
            instances1 = instances1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists information about instances in a project.");
            instances1 = instances1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("partial_update_instance").about("Partially updates an instance within a project. This method can modify all\nfields of an Instance and is the preferred way to update an Instance.");
            instances1 = instances1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on an instance resource. Replaces any\nexisting policy.");
            instances1 = instances1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about(
                "Returns permissions that the caller has on the specified instance resource.",
            );
            instances1 = instances1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates an instance within a project. This method updates only the display\nname and type for an Instance. To update other Instance properties, such as\nlabels, use PartialUpdateInstance.");
            instances1 = instances1.subcommand(mcmd);
        }
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
        let mut operations2 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request. If the\nserver doesn\'t support this method, it returns `UNIMPLEMENTED`.\n\nNOTE: the `name` binding allows API services to override the binding\nto use different resource name schemes, such as `users/*/operations`. To\noverride the binding, API services can add a binding such as\n`\"/v1/{name=users/*}/operations\"` to their service configuration.\nFor backwards compatibility, the default name includes the operations\ncollection id, however overriding users must ensure the name binding\nis the parent resource, without the operations collection id.");
            operations2 = operations2.subcommand(mcmd);
        }
        let mut app_profiles2 = SubCommand::with_name("app_profiles")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd =
                SubCommand::with_name("create").about("Creates an app profile within an instance.");
            app_profiles2 = app_profiles2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes an app profile from an instance.");
            app_profiles2 = app_profiles2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets information about an app profile.");
            app_profiles2 = app_profiles2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists information about app profiles in an instance.");
            app_profiles2 = app_profiles2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("patch").about("Updates an app profile within an instance.");
            app_profiles2 = app_profiles2.subcommand(mcmd);
        }
        let mut clusters2 = SubCommand::with_name("clusters")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and update");
        {
            let mcmd =
                SubCommand::with_name("create").about("Creates a cluster within an instance.");
            clusters2 = clusters2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a cluster from an instance.");
            clusters2 = clusters2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets information about a cluster.");
            clusters2 = clusters2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists information about clusters in an instance.");
            clusters2 = clusters2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("update").about("Updates a cluster within an instance.");
            clusters2 = clusters2.subcommand(mcmd);
        }
        let mut tables2 = SubCommand::with_name("tables")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: check_consistency, create, delete, drop_row_range, generate_consistency_token, get, get_iam_policy, list, modify_column_families, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("check_consistency").about("Checks replication consistency based on a consistency token, that is, if\nreplication has caught up based on the conditions specified in the token\nand the check request.");
            tables2 = tables2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new table in the specified instance.\nThe table can be created with a full set of initial column families,\nspecified in the request.");
            tables2 = tables2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Permanently deletes a specified table and all of its data.");
            tables2 = tables2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("drop_row_range").about("Permanently drop/delete a row range from a specified table. The request can\nspecify whether to delete all rows in a table, or only those that match a\nparticular prefix.");
            tables2 = tables2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("generate_consistency_token").about("Generates a consistency token for a Table, which can be used in\nCheckConsistency to check whether mutations to the table that finished\nbefore this call started have been replicated. The tokens will be available\nfor 90 days.");
            tables2 = tables2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets metadata information about the specified table.");
            tables2 = tables2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a Table resource.\nReturns an empty policy if the resource exists but does not have a policy\nset.");
            tables2 = tables2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all tables served from a specified instance.");
            tables2 = tables2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("modify_column_families").about("Performs a series of column family modifications on the specified table.\nEither all or none of the modifications will occur before this method\nreturns, but data requests received prior to that point may see a table\nwhere only some modifications have taken effect.");
            tables2 = tables2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on a Table resource.\nReplaces any existing policy.");
            tables2 = tables2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions")
                .about("Returns permissions that the caller has on the specified table resource.");
            tables2 = tables2.subcommand(mcmd);
        }
        let mut backups3 = SubCommand::with_name("backups")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get_iam_policy, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a Table resource.\nReturns an empty policy if the resource exists but does not have a policy\nset.");
            backups3 = backups3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on a Table resource.\nReplaces any existing policy.");
            backups3 = backups3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions")
                .about("Returns permissions that the caller has on the specified table resource.");
            backups3 = backups3.subcommand(mcmd);
        }
        clusters2 = clusters2.subcommand(backups3);
        instances1 = instances1.subcommand(tables2);
        instances1 = instances1.subcommand(clusters2);
        instances1 = instances1.subcommand(app_profiles2);
        projects1 = projects1.subcommand(operations2);
        projects0 = projects0.subcommand(locations1);
        projects0 = projects0.subcommand(instances1);
        operations0 = operations0.subcommand(projects1);
        app = app.subcommand(projects0);
        app = app.subcommand(operations0);

        Self { app }
    }
}
use google_bigtableadmin2 as api;

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
