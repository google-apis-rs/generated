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
        let mut app = App::new("youtubereporting1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200505")
            .about("Schedules reporting jobs containing your YouTube Analytics data and downloads the resulting bulk data reports in the form of CSV files.")
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
        let mut jobs0 = SubCommand::with_name("jobs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a job and returns it.");
            jobs0 = jobs0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a job.");
            jobs0 = jobs0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a job.");
            jobs0 = jobs0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists jobs.");
            jobs0 = jobs0.subcommand(mcmd);
        }
        let mut media0 = SubCommand::with_name("media")
            .setting(AppSettings::ColoredHelp)
            .about("methods: download");
        {
            let mcmd = SubCommand::with_name("download").about("Method for media download. Download is supported\non the URI `/v1/media/{+name}?alt=media`.");
            media0 = media0.subcommand(mcmd);
        }
        let mut report_types0 = SubCommand::with_name("report_types")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists report types.");
            report_types0 = report_types0.subcommand(mcmd);
        }
        let mut reports1 = SubCommand::with_name("reports")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets the metadata of a specific report.");
            reports1 = reports1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists reports created by a specific job.\nReturns NOT_FOUND if the job does not exist.");
            reports1 = reports1.subcommand(mcmd);
        }
        jobs0 = jobs0.subcommand(reports1);
        app = app.subcommand(report_types0);
        app = app.subcommand(media0);
        app = app.subcommand(jobs0);

        Self { app }
    }
}
use google_youtubereporting1 as api;

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
