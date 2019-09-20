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
        let mut app = App::new("cloudbuild1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20190824")
            .about("Creates and manages builds on Google Cloud Platform.")
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
            .about("methods: cancel, get and list");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running operation.  The server\nmakes a best effort to cancel the operation, but success is not\nguaranteed.  If the server doesn\'t support this method, it returns\n`google.rpc.Code.UNIMPLEMENTED`.  Clients can use\nOperations.GetOperation or\nother methods to check whether the cancellation succeeded or whether the\noperation completed despite cancellation. On successful cancellation,\nthe operation is not deleted; instead, it becomes an operation with\nan Operation.error value with a google.rpc.Status.code of 1,\ncorresponding to `Code.CANCELLED`.");
            operations0 = operations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation.  Clients can use this\nmethod to poll the operation result at intervals as recommended by the API\nservice.");
            operations0 = operations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request. If the\nserver doesn\'t support this method, it returns `UNIMPLEMENTED`.\n\nNOTE: the `name` binding allows API services to override the binding\nto use different resource name schemes, such as `users/*/operations`. To\noverride the binding, API services can add a binding such as\n`\"/v1/{name=users/*}/operations\"` to their service configuration.\nFor backwards compatibility, the default name includes the operations\ncollection id, however overriding users must ensure the name binding\nis the parent resource, without the operations collection id.");
            operations0 = operations0.subcommand(mcmd);
        }
        let mut projects0 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: builds and triggers");
        let mut builds1 = SubCommand::with_name("builds")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, create, get, list and retry");
        {
            let mcmd = SubCommand::with_name("cancel").about("Cancels a build in progress.");
            builds1 = builds1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Starts a build with the specified configuration.\n\nThis method returns a long-running `Operation`, which includes the build\nID. Pass the build ID to `GetBuild` to determine the build status (such as\n`SUCCESS` or `FAILURE`).");
            builds1 = builds1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns information about a previously requested build.\n\nThe `Build` that is returned includes its status (such as `SUCCESS`,\n`FAILURE`, or `WORKING`), and timing information.");
            builds1 = builds1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists previously requested builds.\n\nPreviously requested builds may still be in-progress, or may have finished\nsuccessfully or unsuccessfully.");
            builds1 = builds1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("retry").about("Creates a new build based on the specified build.\n\nThis method creates a new build using the original build request, which may\nor may not result in an identical build.\n\nFor triggered builds:\n\n* Triggered builds resolve to a precise revision; therefore a retry of a\ntriggered build will result in a build that uses the same revision.\n\nFor non-triggered builds that specify `RepoSource`:\n\n* If the original build built from the tip of a branch, the retried build\nwill build from the tip of that branch, which may not be the same revision\nas the original build.\n* If the original build specified a commit sha or revision ID, the retried\nbuild will use the identical source.\n\nFor builds that specify `StorageSource`:\n\n* If the original build pulled source from Google Cloud Storage without\nspecifying the generation of the object, the new build will use the current\nobject, which may be different from the original build source.\n* If the original build pulled source from Cloud Storage and specified the\ngeneration of the object, the new build will attempt to use the same\nobject, which may or may not be available depending on the bucket\'s\nlifecycle management settings.");
            builds1 = builds1.subcommand(mcmd);
        }
        let mut triggers1 = SubCommand::with_name("triggers")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, patch and run");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a new `BuildTrigger`.\n\nThis API is experimental.");
            triggers1 = triggers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a `BuildTrigger` by its project ID and trigger ID.\n\nThis API is experimental.");
            triggers1 = triggers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Returns information about a `BuildTrigger`.\n\nThis API is experimental.");
            triggers1 = triggers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists existing `BuildTrigger`s.\n\nThis API is experimental.");
            triggers1 = triggers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a `BuildTrigger` by its project ID and trigger ID.\n\nThis API is experimental.");
            triggers1 = triggers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("run")
                .about("Runs a `BuildTrigger` at a particular source revision.");
            triggers1 = triggers1.subcommand(mcmd);
        }
        projects0 = projects0.subcommand(triggers1);
        projects0 = projects0.subcommand(builds1);
        app = app.subcommand(projects0);
        app = app.subcommand(operations0);

        Self { app }
    }
}
use google_cloudbuild1 as api;

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
