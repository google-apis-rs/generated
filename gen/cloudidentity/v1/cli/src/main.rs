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
        let mut app = App::new("cloudidentity1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200509")
            .about("API for provisioning and managing identity resources.")
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
        let mut groups0 = SubCommand::with_name("groups")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, lookup, patch and search");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a Group.");
            groups0 = groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a Group.");
            groups0 = groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves a Group.");
            groups0 = groups0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists groups within a customer or a domain.");
            groups0 = groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("lookup").about("Looks up [resource\nname](https://cloud.google.com/apis/design/resource_names) of a Group by\nits EntityKey.");
            groups0 = groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a Group.");
            groups0 = groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("search").about("Searches for Groups.");
            groups0 = groups0.subcommand(mcmd);
        }
        let mut memberships1 = SubCommand::with_name("memberships")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and lookup");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a Membership.");
            memberships1 = memberships1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a Membership.");
            memberships1 = memberships1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves a Membership.");
            memberships1 = memberships1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists Memberships within a Group.");
            memberships1 = memberships1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("lookup").about("Looks up [resource\nname](https://cloud.google.com/apis/design/resource_names) of a Membership\nwithin a Group by member\'s EntityKey.");
            memberships1 = memberships1.subcommand(mcmd);
        }
        groups0 = groups0.subcommand(memberships1);
        app = app.subcommand(groups0);

        Self { app }
    }
}
use google_cloudidentity1 as api;

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
