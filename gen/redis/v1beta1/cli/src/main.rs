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
        let mut app = App::new("redis1_beta1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200623")
            .about("Creates and manages Redis instances on the Google Cloud Platform.")
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
        let mut instances2 = SubCommand::with_name("instances")
            .setting(AppSettings::ColoredHelp)
            .about(
                "methods: create, delete, export, failover, get, import, list, patch and upgrade",
            );
        {
            let mcmd = SubCommand::with_name("create").about("Creates a Redis instance based on the specified tier and memory size.\n\nBy default, the instance is accessible from the project\'s\n[default network](https://cloud.google.com/vpc/docs/vpc).\n\nThe creation is executed asynchronously and callers may check the returned\noperation to track its progress. Once the operation is completed the Redis\ninstance will be fully functional. Completed longrunning.Operation will\ncontain the new instance object in the response field.\n\nThe returned operation is automatically deleted after a few hours, so there\nis no need to call DeleteOperation.");
            instances2 = instances2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about(
                "Deletes a specific Redis instance.  Instance stops serving and data is\ndeleted.",
            );
            instances2 = instances2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("export").about("Export Redis instance data into a Redis RDB format file in Cloud Storage.\n\nRedis will continue serving during this operation.\n\nThe returned operation is automatically deleted after a few hours, so\nthere is no need to call DeleteOperation.");
            instances2 = instances2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("failover").about("Initiates a failover of the master node to current replica node for a\nspecific STANDARD tier Cloud Memorystore for Redis instance.");
            instances2 = instances2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets the details of a specific Redis instance.");
            instances2 = instances2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("import").about("Import a Redis RDB snapshot file from Cloud Storage into a Redis instance.\n\nRedis may stop serving during this operation. Instance state will be\nIMPORTING for entire operation. When complete, the instance will contain\nonly data from the imported file.\n\nThe returned operation is automatically deleted after a few hours, so\nthere is no need to call DeleteOperation.");
            instances2 = instances2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all Redis instances owned by a project in either the specified\nlocation (region) or all locations.\n\nThe location should have the following format:\n\n* `projects/{project_id}/locations/{location_id}`\n\nIf `location_id` is specified as `-` (wildcard), then all regions\navailable to the project are queried, and the results are aggregated.");
            instances2 = instances2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the metadata and configuration of a specific Redis instance.\n\nCompleted longrunning.Operation will contain the new instance object\nin the response field. The returned operation is automatically deleted\nafter a few hours, so there is no need to call DeleteOperation.");
            instances2 = instances2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("upgrade").about(
                "Upgrades Redis instance to the newer Redis version specified in the\nrequest.",
            );
            instances2 = instances2.subcommand(mcmd);
        }
        let mut operations2 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, delete, get and list");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running operation.  The server\nmakes a best effort to cancel the operation, but success is not\nguaranteed.  If the server doesn\'t support this method, it returns\n`google.rpc.Code.UNIMPLEMENTED`.  Clients can use\nOperations.GetOperation or\nother methods to check whether the cancellation succeeded or whether the\noperation completed despite cancellation. On successful cancellation,\nthe operation is not deleted; instead, it becomes an operation with\nan Operation.error value with a google.rpc.Status.code of 1,\ncorresponding to `Code.CANCELLED`.");
            operations2 = operations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a long-running operation. This method indicates that the client is\nno longer interested in the operation result. It does not cancel the\noperation. If the server doesn\'t support this method, it returns\n`google.rpc.Code.UNIMPLEMENTED`.");
            operations2 = operations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation.  Clients can use this\nmethod to poll the operation result at intervals as recommended by the API\nservice.");
            operations2 = operations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request. If the\nserver doesn\'t support this method, it returns `UNIMPLEMENTED`.\n\nNOTE: the `name` binding allows API services to override the binding\nto use different resource name schemes, such as `users/*/operations`. To\noverride the binding, API services can add a binding such as\n`\"/v1/{name=users/*}/operations\"` to their service configuration.\nFor backwards compatibility, the default name includes the operations\ncollection id, however overriding users must ensure the name binding\nis the parent resource, without the operations collection id.");
            operations2 = operations2.subcommand(mcmd);
        }
        locations1 = locations1.subcommand(operations2);
        locations1 = locations1.subcommand(instances2);
        projects0 = projects0.subcommand(locations1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_redis1_beta1 as api;

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
