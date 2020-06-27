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
        let mut app = App::new("dns1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200515")
            .about("Configures and serves authoritative DNS records.")
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
        let mut changes0 = SubCommand::with_name("changes")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, get and list");
        {
            let mcmd = SubCommand::with_name("create").about("");
            changes0 = changes0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("");
            changes0 = changes0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("");
            changes0 = changes0.subcommand(mcmd);
        }
        let mut dns_keys0 = SubCommand::with_name("dns_keys")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("");
            dns_keys0 = dns_keys0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("");
            dns_keys0 = dns_keys0.subcommand(mcmd);
        }
        let mut managed_zone_operations0 = SubCommand::with_name("managed_zone_operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("");
            managed_zone_operations0 = managed_zone_operations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("");
            managed_zone_operations0 = managed_zone_operations0.subcommand(mcmd);
        }
        let mut managed_zones0 = SubCommand::with_name("managed_zones")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, patch and update");
        {
            let mcmd = SubCommand::with_name("create").about("");
            managed_zones0 = managed_zones0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("");
            managed_zones0 = managed_zones0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("");
            managed_zones0 = managed_zones0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("");
            managed_zones0 = managed_zones0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("");
            managed_zones0 = managed_zones0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("");
            managed_zones0 = managed_zones0.subcommand(mcmd);
        }
        let mut policies0 = SubCommand::with_name("policies")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, patch and update");
        {
            let mcmd = SubCommand::with_name("create").about("");
            policies0 = policies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("");
            policies0 = policies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("");
            policies0 = policies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("");
            policies0 = policies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("");
            policies0 = policies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("");
            policies0 = policies0.subcommand(mcmd);
        }
        let mut projects0 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("");
            projects0 = projects0.subcommand(mcmd);
        }
        let mut resource_record_sets0 = SubCommand::with_name("resource_record_sets")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("");
            resource_record_sets0 = resource_record_sets0.subcommand(mcmd);
        }
        app = app.subcommand(resource_record_sets0);
        app = app.subcommand(projects0);
        app = app.subcommand(policies0);
        app = app.subcommand(managed_zones0);
        app = app.subcommand(managed_zone_operations0);
        app = app.subcommand(dns_keys0);
        app = app.subcommand(changes0);

        Self { app }
    }
}
use google_dns1 as api;

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
