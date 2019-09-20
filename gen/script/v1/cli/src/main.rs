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
        let mut app = App::new("script1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20190912")
            .about("Manages and executes Google Apps Script projects.\n")
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
        let mut processes0 = SubCommand::with_name("processes")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list and list_script_processes");
        {
            let mcmd = SubCommand::with_name("list").about("List information about processes made by or on behalf of a user,\nsuch as process type and current status.");
            processes0 = processes0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list_script_processes").about("List information about a script\'s executed processes, such as process type\nand current status.");
            processes0 = processes0.subcommand(mcmd);
        }
        let mut projects0 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, get, get_content, get_metrics and update_content");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new, empty script project with no script files and a base\nmanifest file.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a script project\'s metadata.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_content").about("Gets the content of the script project, including the code source and\nmetadata for each script file.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_metrics").about(
                "Get metrics data for scripts, such as number of executions and\nactive users.",
            );
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_content").about("Updates the content of the specified script project.\nThis content is stored as the HEAD version, and is used when the script is\nexecuted as a trigger, in the script editor, in add-on preview mode, or as\na web app or Apps Script API in development mode. This clears all the\nexisting files in the project.");
            projects0 = projects0.subcommand(mcmd);
        }
        let mut scripts0 = SubCommand::with_name("scripts")
            .setting(AppSettings::ColoredHelp)
            .about("methods: run");
        {
            let mcmd = SubCommand::with_name("run").about("Runs a function in an Apps Script project. The script project must be\ndeployed for use with the Apps Script API and the calling application must\nshare the same Cloud Platform project.\n\nThis method requires authorization with an OAuth 2.0 token that includes at\nleast one of the scopes listed in the\n[Authorization](#authorization-scopes) section; script projects that do not\nrequire authorization cannot be executed through this API. To find the\ncorrect scopes to include in the authentication token, open the project in\nthe script editor, then select **File > Project properties** and click the\n**Scopes** tab.\n\nThe error `403, PERMISSION_DENIED: The caller does not have permission`\nindicates that the Cloud Platform project used to authorize the request is\nnot the same as the one used by the script.");
            scripts0 = scripts0.subcommand(mcmd);
        }
        let mut deployments1 = SubCommand::with_name("deployments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and update");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a deployment of an Apps Script project.");
            deployments1 = deployments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes a deployment of an Apps Script project.");
            deployments1 = deployments1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets a deployment of an Apps Script project.");
            deployments1 = deployments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists the deployments of an Apps Script project.");
            deployments1 = deployments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update")
                .about("Updates a deployment of an Apps Script project.");
            deployments1 = deployments1.subcommand(mcmd);
        }
        let mut versions1 = SubCommand::with_name("versions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, get and list");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new immutable version using the current code, with a unique\nversion number.");
            versions1 = versions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a version of a script project.");
            versions1 = versions1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("List the versions of a script project.");
            versions1 = versions1.subcommand(mcmd);
        }
        projects0 = projects0.subcommand(versions1);
        projects0 = projects0.subcommand(deployments1);
        app = app.subcommand(scripts0);
        app = app.subcommand(projects0);
        app = app.subcommand(processes0);

        Self { app }
    }
}
use google_script1 as api;

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
