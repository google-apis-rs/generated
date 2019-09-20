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
        let mut app = App::new("replicapool1_beta1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20160512")
            .about("The Replica Pool API allows users to declaratively provision and manage groups of Google Compute Engine instances based on a common template.")
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
        let mut pools0 = SubCommand::with_name("pools")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, resize and updatetemplate");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a replica pool.");
            pools0 = pools0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets information about a single replica pool.");
            pools0 = pools0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Inserts a new replica pool.");
            pools0 = pools0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List all replica pools.");
            pools0 = pools0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("resize").about("Resize a pool. This is an asynchronous operation, and multiple overlapping resize requests can be made. Replica Pools will use the information from the last resize request.");
            pools0 = pools0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("updatetemplate")
                .about("Update the template used by the pool.");
            pools0 = pools0.subcommand(mcmd);
        }
        let mut replicas0 = SubCommand::with_name("replicas")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, list and restart");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a replica from the pool.");
            replicas0 = replicas0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets information about a specific replica.");
            replicas0 = replicas0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all replicas in a pool.");
            replicas0 = replicas0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("restart").about("Restarts a replica in a pool.");
            replicas0 = replicas0.subcommand(mcmd);
        }
        app = app.subcommand(replicas0);
        app = app.subcommand(pools0);

        Self { app }
    }
}
use google_replicapool1_beta1 as api;

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
