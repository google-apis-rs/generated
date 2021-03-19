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
        let mut app = App::new("datamigration1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20210307")
            .about("Manage Cloud Database Migration Service resources on Google Cloud Platform.")
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
        let mut connection_profiles2 = SubCommand::with_name("connection_profiles")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_iam_policy, list, patch, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a new connection profile in a given project and location.");
            connection_profiles2 = connection_profiles2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a single Database Migration Service connection profile. A connection profile can only be deleted if it is not in use by any active migration jobs.");
            connection_profiles2 = connection_profiles2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets details of a single connection profile.");
            connection_profiles2 = connection_profiles2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.");
            connection_profiles2 = connection_profiles2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Retrieve a list of all connection profiles in a given project and location.",
            );
            connection_profiles2 = connection_profiles2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Update the configuration of a single connection profile.");
            connection_profiles2 = connection_profiles2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.");
            connection_profiles2 = connection_profiles2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may \"fail open\" without warning.");
            connection_profiles2 = connection_profiles2.subcommand(mcmd);
        }
        let mut migration_jobs2 = SubCommand::with_name("migration_jobs")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, generate_ssh_script, get, get_iam_policy, list, patch, promote, restart, resume, set_iam_policy, start, stop, test_iam_permissions and verify");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a new migration job in a given project and location.");
            migration_jobs2 = migration_jobs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a single migration job.");
            migration_jobs2 = migration_jobs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("generate_ssh_script").about(
                "Generate a SSH configuration script to configure the reverse SSH connectivity.",
            );
            migration_jobs2 = migration_jobs2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets details of a single migration job.");
            migration_jobs2 = migration_jobs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.");
            migration_jobs2 = migration_jobs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists migration jobs in a given project and location.");
            migration_jobs2 = migration_jobs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates the parameters of a single migration job.");
            migration_jobs2 = migration_jobs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("promote").about("Promote a migration job, stopping replication to the destination and promoting the destination to be a standalone database.");
            migration_jobs2 = migration_jobs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("restart").about("Restart a stopped or failed migration job, resetting the destination instance to its original state and starting the migration process from scratch.");
            migration_jobs2 = migration_jobs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("resume").about("Resume a migration job that is currently stopped and is resumable (was stopped during CDC phase).");
            migration_jobs2 = migration_jobs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.");
            migration_jobs2 = migration_jobs2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("start").about("Start an already created migration job.");
            migration_jobs2 = migration_jobs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("stop").about("Stops a running migration job.");
            migration_jobs2 = migration_jobs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may \"fail open\" without warning.");
            migration_jobs2 = migration_jobs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("verify").about("Verify a migration job, making sure the destination can reach the source and that all configuration and prerequisites are met.");
            migration_jobs2 = migration_jobs2.subcommand(mcmd);
        }
        let mut operations2 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, delete, get and list");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn\'t support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.");
            operations2 = operations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a long-running operation. This method indicates that the client is no longer interested in the operation result. It does not cancel the operation. If the server doesn\'t support this method, it returns `google.rpc.Code.UNIMPLEMENTED`.");
            operations2 = operations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations2 = operations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request. If the server doesn\'t support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `\"/v1/{name=users/*}/operations\"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.");
            operations2 = operations2.subcommand(mcmd);
        }
        locations1 = locations1.subcommand(operations2);
        locations1 = locations1.subcommand(migration_jobs2);
        locations1 = locations1.subcommand(connection_profiles2);
        projects0 = projects0.subcommand(locations1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_datamigration1 as api;

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
