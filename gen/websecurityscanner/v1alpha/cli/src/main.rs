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
        let mut app = App::new("websecurityscanner1_alpha")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200411")
            .about("Scans your Compute and App Engine apps for common web vulnerabilities.")
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
            .about("sub-resources: scan_configs");
        let mut scan_configs1 = SubCommand::with_name("scan_configs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, patch and start");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new ScanConfig.");
            scan_configs1 = scan_configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes an existing ScanConfig and its child resources.");
            scan_configs1 = scan_configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a ScanConfig.");
            scan_configs1 = scan_configs1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists ScanConfigs under a given project.");
            scan_configs1 = scan_configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates a ScanConfig. This method support partial update of a ScanConfig.");
            scan_configs1 = scan_configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("start")
                .about("Start a ScanRun according to the given ScanConfig.");
            scan_configs1 = scan_configs1.subcommand(mcmd);
        }
        let mut scan_runs2 = SubCommand::with_name("scan_runs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, list and stop");
        {
            let mcmd = SubCommand::with_name("get").about("Gets a ScanRun.");
            scan_runs2 = scan_runs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists ScanRuns under a given ScanConfig, in descending order of ScanRun\nstop time.");
            scan_runs2 = scan_runs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("stop")
                .about("Stops a ScanRun. The stopped ScanRun is returned.");
            scan_runs2 = scan_runs2.subcommand(mcmd);
        }
        let mut crawled_urls3 = SubCommand::with_name("crawled_urls")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd =
                SubCommand::with_name("list").about("List CrawledUrls under a given ScanRun.");
            crawled_urls3 = crawled_urls3.subcommand(mcmd);
        }
        let mut finding_type_stats3 = SubCommand::with_name("finding_type_stats")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("List all FindingTypeStats under a given ScanRun.");
            finding_type_stats3 = finding_type_stats3.subcommand(mcmd);
        }
        let mut findings3 = SubCommand::with_name("findings")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets a Finding.");
            findings3 = findings3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List Findings under a given ScanRun.");
            findings3 = findings3.subcommand(mcmd);
        }
        scan_runs2 = scan_runs2.subcommand(findings3);
        scan_runs2 = scan_runs2.subcommand(finding_type_stats3);
        scan_runs2 = scan_runs2.subcommand(crawled_urls3);
        scan_configs1 = scan_configs1.subcommand(scan_runs2);
        projects0 = projects0.subcommand(scan_configs1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_websecurityscanner1_alpha as api;

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
