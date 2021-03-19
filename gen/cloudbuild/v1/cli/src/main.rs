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
            .version("0.1.0-20210313")
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
            .about("methods: cancel and get");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn\'t support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.");
            operations0 = operations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations0 = operations0.subcommand(mcmd);
        }
        let mut projects0 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: builds, locations and triggers");
        let mut builds1 = SubCommand::with_name("builds")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, create, get, list and retry");
        {
            let mcmd = SubCommand::with_name("cancel").about("Cancels a build in progress.");
            builds1 = builds1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Starts a build with the specified configuration. This method returns a long-running `Operation`, which includes the build ID. Pass the build ID to `GetBuild` to determine the build status (such as `SUCCESS` or `FAILURE`).");
            builds1 = builds1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns information about a previously requested build. The `Build` that is returned includes its status (such as `SUCCESS`, `FAILURE`, or `WORKING`), and timing information.");
            builds1 = builds1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists previously requested builds. Previously requested builds may still be in-progress, or may have finished successfully or unsuccessfully.");
            builds1 = builds1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("retry").about("Creates a new build based on the specified build. This method creates a new build using the original build request, which may or may not result in an identical build. For triggered builds: * Triggered builds resolve to a precise revision; therefore a retry of a triggered build will result in a build that uses the same revision. For non-triggered builds that specify `RepoSource`: * If the original build built from the tip of a branch, the retried build will build from the tip of that branch, which may not be the same revision as the original build. * If the original build specified a commit sha or revision ID, the retried build will use the identical source. For builds that specify `StorageSource`: * If the original build pulled source from Google Cloud Storage without specifying the generation of the object, the new build will use the current object, which may be different from the original build source. * If the original build pulled source from Cloud Storage and specified the generation of the object, the new build will attempt to use the same object, which may or may not be available depending on the bucket\'s lifecycle management settings.");
            builds1 = builds1.subcommand(mcmd);
        }
        let mut locations1 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: builds and operations");
        let mut triggers1 = SubCommand::with_name("triggers")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, patch, run and webhook");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a new `BuildTrigger`. This API is experimental.");
            triggers1 = triggers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a `BuildTrigger` by its project ID and trigger ID. This API is experimental.");
            triggers1 = triggers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Returns information about a `BuildTrigger`. This API is experimental.");
            triggers1 = triggers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists existing `BuildTrigger`s. This API is experimental.");
            triggers1 = triggers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a `BuildTrigger` by its project ID and trigger ID. This API is experimental.");
            triggers1 = triggers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("run")
                .about("Runs a `BuildTrigger` at a particular source revision.");
            triggers1 = triggers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("webhook").about("ReceiveTriggerWebhook [Experimental] is called when the API receives a webhook request targeted at a specific trigger.");
            triggers1 = triggers1.subcommand(mcmd);
        }
        let mut builds2 = SubCommand::with_name("builds")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, create, get, list and retry");
        {
            let mcmd = SubCommand::with_name("cancel").about("Cancels a build in progress.");
            builds2 = builds2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Starts a build with the specified configuration. This method returns a long-running `Operation`, which includes the build ID. Pass the build ID to `GetBuild` to determine the build status (such as `SUCCESS` or `FAILURE`).");
            builds2 = builds2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns information about a previously requested build. The `Build` that is returned includes its status (such as `SUCCESS`, `FAILURE`, or `WORKING`), and timing information.");
            builds2 = builds2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists previously requested builds. Previously requested builds may still be in-progress, or may have finished successfully or unsuccessfully.");
            builds2 = builds2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("retry").about("Creates a new build based on the specified build. This method creates a new build using the original build request, which may or may not result in an identical build. For triggered builds: * Triggered builds resolve to a precise revision; therefore a retry of a triggered build will result in a build that uses the same revision. For non-triggered builds that specify `RepoSource`: * If the original build built from the tip of a branch, the retried build will build from the tip of that branch, which may not be the same revision as the original build. * If the original build specified a commit sha or revision ID, the retried build will use the identical source. For builds that specify `StorageSource`: * If the original build pulled source from Google Cloud Storage without specifying the generation of the object, the new build will use the current object, which may be different from the original build source. * If the original build pulled source from Cloud Storage and specified the generation of the object, the new build will attempt to use the same object, which may or may not be available depending on the bucket\'s lifecycle management settings.");
            builds2 = builds2.subcommand(mcmd);
        }
        let mut operations2 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel and get");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn\'t support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.");
            operations2 = operations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations2 = operations2.subcommand(mcmd);
        }
        locations1 = locations1.subcommand(operations2);
        locations1 = locations1.subcommand(builds2);
        projects0 = projects0.subcommand(triggers1);
        projects0 = projects0.subcommand(locations1);
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
