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
        let mut app = App::new("youtubeAnalytics2")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200624")
            .about("Retrieves your YouTube Analytics data.")
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
        let mut group_items0 = SubCommand::with_name("group_items")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, insert and list");
        {
            let mcmd = SubCommand::with_name("delete").about("Removes an item from a group.");
            group_items0 = group_items0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a group item.");
            group_items0 = group_items0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Returns a collection of group items that match the API request parameters.",
            );
            group_items0 = group_items0.subcommand(mcmd);
        }
        let mut groups0 = SubCommand::with_name("groups")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, insert, list and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a group.");
            groups0 = groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a group.");
            groups0 = groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Returns a collection of groups that match the API request parameters. For\nexample, you can retrieve all groups that the authenticated user owns,\nor you can retrieve one or more groups by their unique IDs.");
            groups0 = groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update")
                .about("Modifies a group. For example, you could change a group\'s title.");
            groups0 = groups0.subcommand(mcmd);
        }
        let mut reports0 = SubCommand::with_name("reports")
            .setting(AppSettings::ColoredHelp)
            .about("methods: query");
        {
            let mcmd =
                SubCommand::with_name("query").about("Retrieve your YouTube Analytics reports.");
            reports0 = reports0.subcommand(mcmd);
        }
        app = app.subcommand(reports0);
        app = app.subcommand(groups0);
        app = app.subcommand(group_items0);

        Self { app }
    }
}
use google_youtubeAnalytics2 as api;

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
