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
        let mut app = App::new("runtimeconfig1_beta1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200622")
            .about("The Runtime Configurator allows you to dynamically configure and expose variables through Google Cloud Platform. In addition, you can also set Watchers and Waiters that will watch for changes to your data and return based on certain conditions.")
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
            .about("sub-resources: configs");
        let mut configs1 = SubCommand::with_name("configs")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_iam_policy, list, set_iam_policy, test_iam_permissions and update");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new RuntimeConfig resource. The configuration name must be\nunique within project.");
            configs1 = configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a RuntimeConfig resource.");
            configs1 = configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets information about a RuntimeConfig resource.");
            configs1 = configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource.\nReturns an empty policy if the resource exists and does not have a policy\nset.");
            configs1 = configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all the RuntimeConfig resources within project.");
            configs1 = configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any\nexisting policy.\n\nCan return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.");
            configs1 = configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource.\nIf the resource does not exist, this will return an empty set of\npermissions, not a `NOT_FOUND` error.\n\nNote: This operation is designed to be used for building permission-aware\nUIs and command-line tools, not for authorization checking. This operation\nmay \"fail open\" without warning.");
            configs1 = configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about(
                "Updates a RuntimeConfig resource. The configuration must exist beforehand.",
            );
            configs1 = configs1.subcommand(mcmd);
        }
        let mut operations2 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation.  Clients can use this\nmethod to poll the operation result at intervals as recommended by the API\nservice.");
            operations2 = operations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource.\nIf the resource does not exist, this will return an empty set of\npermissions, not a `NOT_FOUND` error.\n\nNote: This operation is designed to be used for building permission-aware\nUIs and command-line tools, not for authorization checking. This operation\nmay \"fail open\" without warning.");
            operations2 = operations2.subcommand(mcmd);
        }
        let mut variables2 = SubCommand::with_name("variables")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, test_iam_permissions, update and watch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a variable within the given configuration. You cannot create\na variable with a name that is a prefix of an existing variable name, or a\nname that has an existing variable name as a prefix.\n\nTo learn more about creating a variable, read the\n[Setting and Getting\nData](/deployment-manager/runtime-configurator/set-and-get-variables)\ndocumentation.");
            variables2 = variables2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a variable or multiple variables.\n\nIf you specify a variable name, then that variable is deleted. If you\nspecify a prefix and `recursive` is true, then all variables with that\nprefix are deleted. You must set a `recursive` to true if you delete\nvariables by prefix.");
            variables2 = variables2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets information about a single variable.");
            variables2 = variables2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists variables within given a configuration, matching any provided\nfilters. This only lists variable names, not the values, unless\n`return_values` is true, in which case only variables that user has IAM\npermission to GetVariable will be returned.");
            variables2 = variables2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource.\nIf the resource does not exist, this will return an empty set of\npermissions, not a `NOT_FOUND` error.\n\nNote: This operation is designed to be used for building permission-aware\nUIs and command-line tools, not for authorization checking. This operation\nmay \"fail open\" without warning.");
            variables2 = variables2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update")
                .about("Updates an existing variable with a new value.");
            variables2 = variables2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("watch").about("Watches a specific variable and waits for a change in the variable\'s value.\nWhen there is a change, this method returns the new value or times out.\n\nIf a variable is deleted while being watched, the `variableState` state is\nset to `DELETED` and the method returns the last known variable `value`.\n\nIf you set the deadline for watching to a larger value than internal\ntimeout (60 seconds), the current variable value is returned and the\n`variableState` will be `VARIABLE_STATE_UNSPECIFIED`.\n\nTo learn more about creating a watcher, read the\n[Watching a Variable for\nChanges](/deployment-manager/runtime-configurator/watching-a-variable)\ndocumentation.");
            variables2 = variables2.subcommand(mcmd);
        }
        let mut waiters2 = SubCommand::with_name("waiters")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a Waiter resource. This operation returns a long-running Operation\nresource which can be polled for completion. However, a waiter with the\ngiven name will exist (and can be retrieved) prior to the operation\ncompleting. If the operation fails, the failed Waiter resource will\nstill exist and must be deleted prior to subsequent creation attempts.");
            waiters2 = waiters2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes the waiter with the specified name.");
            waiters2 = waiters2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets information about a single waiter.");
            waiters2 = waiters2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("List waiters within the given configuration.");
            waiters2 = waiters2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource.\nIf the resource does not exist, this will return an empty set of\npermissions, not a `NOT_FOUND` error.\n\nNote: This operation is designed to be used for building permission-aware\nUIs and command-line tools, not for authorization checking. This operation\nmay \"fail open\" without warning.");
            waiters2 = waiters2.subcommand(mcmd);
        }
        configs1 = configs1.subcommand(waiters2);
        configs1 = configs1.subcommand(variables2);
        configs1 = configs1.subcommand(operations2);
        projects0 = projects0.subcommand(configs1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_runtimeconfig1_beta1 as api;

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
