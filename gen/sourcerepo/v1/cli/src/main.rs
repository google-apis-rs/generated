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
        let mut app = App::new("sourcerepo1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20190611")
            .about("Accesses source code repositories hosted by Google.")
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
            .about("methods: get_config and update_config");
        {
            let mcmd = SubCommand::with_name("get_config")
                .about("Returns the Cloud Source Repositories configuration of the project.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_config")
                .about("Updates the Cloud Source Repositories configuration of the project.");
            projects0 = projects0.subcommand(mcmd);
        }
        let mut repos1 = SubCommand::with_name("repos")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_iam_policy, list, patch, set_iam_policy, sync and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a repo in the given project with the given name.\n\nIf the named repository already exists, `CreateRepo` returns\n`ALREADY_EXISTS`.");
            repos1 = repos1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a repo.");
            repos1 = repos1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns information about a repo.");
            repos1 = repos1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource.\nReturns an empty policy if the resource exists and does not have a policy\nset.");
            repos1 = repos1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Returns all repos belonging to a project. The sizes of the repos are\nnot set by ListRepos.  To get the size of a repo, use GetRepo.");
            repos1 = repos1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates information about a repo.");
            repos1 = repos1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any\nexisting policy.");
            repos1 = repos1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("sync").about("Synchronize a connected repo.\n\nThe response contains SyncRepoMetadata in the metadata field.");
            repos1 = repos1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource.\nIf the resource does not exist, this will return an empty set of\npermissions, not a NOT_FOUND error.");
            repos1 = repos1.subcommand(mcmd);
        }
        projects0 = projects0.subcommand(repos1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_sourcerepo1 as api;

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
