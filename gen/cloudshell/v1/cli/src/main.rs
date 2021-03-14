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
        let mut app = App::new("cloudshell1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20210310")
            .about("Allows users to start, configure, and connect to interactive shell sessions running in the cloud. ")
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
            .about("methods: cancel, delete, get and list");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn\'t support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.");
            operations0 = operations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a long-running operation. This method indicates that the client is no longer interested in the operation result. It does not cancel the operation. If the server doesn\'t support this method, it returns `google.rpc.Code.UNIMPLEMENTED`.");
            operations0 = operations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations0 = operations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request. If the server doesn\'t support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `\"/v1/{name=users/*}/operations\"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.");
            operations0 = operations0.subcommand(mcmd);
        }
        let mut users0 = SubCommand::with_name("users")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: environments");
        let mut environments1 = SubCommand::with_name("environments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: add_public_key, authorize, get, remove_public_key and start");
        {
            let mcmd = SubCommand::with_name("add_public_key").about("Adds a public SSH key to an environment, allowing clients with the corresponding private key to connect to that environment via SSH. If a key with the same content already exists, this will error with ALREADY_EXISTS.");
            environments1 = environments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("authorize").about("Sends OAuth credentials to a running environment on behalf of a user. When this completes, the environment will be authorized to run various Google Cloud command line tools without requiring the user to manually authenticate.");
            environments1 = environments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets an environment. Returns NOT_FOUND if the environment does not exist.");
            environments1 = environments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("remove_public_key").about("Removes a public SSH key from an environment. Clients will no longer be able to connect to the environment using the corresponding private key. If a key with the same content is not present, this will error with NOT_FOUND.");
            environments1 = environments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("start").about("Starts an existing environment, allowing clients to connect to it. The returned operation will contain an instance of StartEnvironmentMetadata in its metadata field. Users can wait for the environment to start by polling this operation via GetOperation. Once the environment has finished starting and is ready to accept connections, the operation will contain a StartEnvironmentResponse in its response field.");
            environments1 = environments1.subcommand(mcmd);
        }
        users0 = users0.subcommand(environments1);
        app = app.subcommand(users0);
        app = app.subcommand(operations0);

        Self { app }
    }
}
use google_cloudshell1 as api;

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
