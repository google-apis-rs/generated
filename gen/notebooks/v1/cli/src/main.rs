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
        let mut app = App::new("notebooks1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20210305")
            .about("AI Platform Notebooks API is used to manage notebook resources in Google Cloud.")
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
        let mut environments2 = SubCommand::with_name("environments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new Environment.");
            environments2 = environments2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a single Environment.");
            environments2 = environments2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets details of a single Environment.");
            environments2 = environments2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists environments in a project.");
            environments2 = environments2.subcommand(mcmd);
        }
        let mut executions2 = SubCommand::with_name("executions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a new Scheduled Notebook in a given project and location.");
            executions2 = executions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes execution");
            executions2 = executions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets details of executions");
            executions2 = executions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists executions in a given project and location");
            executions2 = executions2.subcommand(mcmd);
        }
        let mut instances2 = SubCommand::with_name("instances")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_iam_policy, get_instance_health, is_upgradeable, list, register, report, reset, set_accelerator, set_iam_policy, set_labels, set_machine_type, start, stop, test_iam_permissions, upgrade and upgrade_internal");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a new Instance in a given project and location.");
            instances2 = instances2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a single Instance.");
            instances2 = instances2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets details of a single Instance.");
            instances2 = instances2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.");
            instances2 = instances2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_instance_health")
                .about("Check if a notebook instance is healthy.");
            instances2 = instances2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("is_upgradeable")
                .about("Check if a notebook instance is upgradable.");
            instances2 = instances2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists instances in a given project and location.");
            instances2 = instances2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("register").about("Registers an existing legacy notebook instance to the Notebooks API server. Legacy instances are instances created with the legacy Compute Engine calls. They are not manageable by the Notebooks API out of the box. This call makes these instances manageable by the Notebooks API.");
            instances2 = instances2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("report").about("Allows notebook instances to report their latest instance information to the Notebooks API server. The server will merge the reported information to the instance metadata store. Do not use this method directly.");
            instances2 = instances2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("reset").about("Resets a notebook instance.");
            instances2 = instances2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_accelerator")
                .about("Updates the guest accelerators of a single Instance.");
            instances2 = instances2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.");
            instances2 = instances2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_labels")
                .about("Replaces all the labels of an Instance.");
            instances2 = instances2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_machine_type")
                .about("Updates the machine type of a single Instance.");
            instances2 = instances2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("start").about("Starts a notebook instance.");
            instances2 = instances2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("stop").about("Stops a notebook instance.");
            instances2 = instances2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may \"fail open\" without warning.");
            instances2 = instances2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("upgrade")
                .about("Upgrades a notebook instance to the latest version.");
            instances2 = instances2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("upgrade_internal").about("Allows notebook instances to call this endpoint to upgrade themselves. Do not use this method directly.");
            instances2 = instances2.subcommand(mcmd);
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
        let mut schedules2 = SubCommand::with_name("schedules")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and trigger");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a new Scheduled Notebook in a given project and location.");
            schedules2 = schedules2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes schedule and all underlying jobs");
            schedules2 = schedules2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets details of schedule");
            schedules2 = schedules2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists schedules in a given project and location.");
            schedules2 = schedules2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("trigger")
                .about("Triggers execution of an existing schedule.");
            schedules2 = schedules2.subcommand(mcmd);
        }
        locations1 = locations1.subcommand(schedules2);
        locations1 = locations1.subcommand(operations2);
        locations1 = locations1.subcommand(instances2);
        locations1 = locations1.subcommand(executions2);
        locations1 = locations1.subcommand(environments2);
        projects0 = projects0.subcommand(locations1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_notebooks1 as api;

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
