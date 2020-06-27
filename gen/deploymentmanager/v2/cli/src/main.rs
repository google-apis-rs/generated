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
        let mut app = App::new("deploymentmanager2")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200512")
            .about("Declares, configures, and deploys complex solutions on Google Cloud Platform.")
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
        let mut deployments0 = SubCommand::with_name("deployments")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: cancel_preview, delete, get, get_iam_policy, insert, list, patch, set_iam_policy, stop, test_iam_permissions and update");
        {
            let mcmd = SubCommand::with_name("cancel_preview")
                .about("Cancels and removes the preview currently associated with the deployment.");
            deployments0 = deployments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes a deployment and all of the resources in the deployment.");
            deployments0 = deployments0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets information about a specific deployment.");
            deployments0 = deployments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. May be empty if no such policy or resource exists.");
            deployments0 = deployments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a deployment and all of the resources described by the deployment manifest.");
            deployments0 = deployments0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists all deployments for a given project.");
            deployments0 = deployments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Patches a deployment and all of the resources described by the deployment manifest.");
            deployments0 = deployments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy.");
            deployments0 = deployments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("stop").about("Stops an ongoing operation. This does not roll back any work that has already been completed, but prevents any new work from being started.");
            deployments0 = deployments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions")
                .about("Returns permissions that a caller has on the specified resource.");
            deployments0 = deployments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a deployment and all of the resources described by the deployment manifest.");
            deployments0 = deployments0.subcommand(mcmd);
        }
        let mut manifests0 = SubCommand::with_name("manifests")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets information about a specific manifest.");
            manifests0 = manifests0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists all manifests for a given deployment.");
            manifests0 = manifests0.subcommand(mcmd);
        }
        let mut operations0 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets information about a specific operation.");
            operations0 = operations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all operations for a project.");
            operations0 = operations0.subcommand(mcmd);
        }
        let mut resources0 = SubCommand::with_name("resources")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets information about a single resource.");
            resources0 = resources0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists all resources in a given deployment.");
            resources0 = resources0.subcommand(mcmd);
        }
        let mut types0 = SubCommand::with_name("types")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all resource types for Deployment Manager.");
            types0 = types0.subcommand(mcmd);
        }
        app = app.subcommand(types0);
        app = app.subcommand(resources0);
        app = app.subcommand(operations0);
        app = app.subcommand(manifests0);
        app = app.subcommand(deployments0);

        Self { app }
    }
}
use google_deploymentmanager2 as api;

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
