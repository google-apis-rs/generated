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
        let mut app = App::new("remotebuildexecution1_alpha")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200505")
            .about("Supplies a Remote Execution API service for tools such as bazel.")
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
            .about("sub-resources: instances and operations");
        let mut instances1 = SubCommand::with_name("instances")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new instance in the specified region.\nReturns a long running operation which contains an instance on completion.\nWhile the long running operation is in progress, any call to `GetInstance`\nreturns an instance in state `CREATING`.");
            instances1 = instances1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified instance.\nReturns a long running operation which contains a `google.protobuf.Empty`\nresponse on completion.\nDeleting an instance with worker pools in it will delete these worker\npools.");
            instances1 = instances1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified instance.");
            instances1 = instances1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists instances in a project.");
            instances1 = instances1.subcommand(mcmd);
        }
        let mut operations1 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation.  Clients can use this\nmethod to poll the operation result at intervals as recommended by the API\nservice.");
            operations1 = operations1.subcommand(mcmd);
        }
        let mut workerpools2 = SubCommand::with_name("workerpools")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new worker pool with a specified size and configuration.\nReturns a long running operation which contains a worker pool on\ncompletion. While the long running operation is in progress, any call to\n`GetWorkerPool` returns a worker pool in state `CREATING`.");
            workerpools2 = workerpools2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified worker pool.\nReturns a long running operation, which contains a `google.protobuf.Empty`\nresponse on completion.\nWhile the long running operation is in progress, any call to\n`GetWorkerPool` returns a worker pool in state `DELETING`.");
            workerpools2 = workerpools2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified worker pool.");
            workerpools2 = workerpools2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists worker pools in an instance.");
            workerpools2 = workerpools2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an existing worker pool with a specified size and/or configuration.\nReturns a long running operation, which contains a worker pool on\ncompletion. While the long running operation is in progress, any call to\n`GetWorkerPool` returns a worker pool in state `UPDATING`.");
            workerpools2 = workerpools2.subcommand(mcmd);
        }
        instances1 = instances1.subcommand(workerpools2);
        projects0 = projects0.subcommand(operations1);
        projects0 = projects0.subcommand(instances1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_remotebuildexecution1_alpha as api;

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
