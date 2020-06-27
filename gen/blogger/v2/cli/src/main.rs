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
        let mut app = App::new("blogger2")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200625")
            .about("The Blogger API provides access to posts, comments and pages of a\n    Blogger blog.")
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
        let mut blogs0 = SubCommand::with_name("blogs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets a blog by id.");
            blogs0 = blogs0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists blogs by user id, possibly filtered.");
            blogs0 = blogs0.subcommand(mcmd);
        }
        let mut comments0 = SubCommand::with_name("comments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets a comment by blog id, post id and comment id.");
            comments0 = comments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists comments.");
            comments0 = comments0.subcommand(mcmd);
        }
        let mut pages0 = SubCommand::with_name("pages")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets a page by blog id and page id.");
            pages0 = pages0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists pages.");
            pages0 = pages0.subcommand(mcmd);
        }
        let mut posts0 = SubCommand::with_name("posts")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets a post by blog id and post id");
            posts0 = posts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists posts.");
            posts0 = posts0.subcommand(mcmd);
        }
        let mut users0 = SubCommand::with_name("users")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Gets a user by user id.");
            users0 = users0.subcommand(mcmd);
        }
        app = app.subcommand(users0);
        app = app.subcommand(posts0);
        app = app.subcommand(pages0);
        app = app.subcommand(comments0);
        app = app.subcommand(blogs0);

        Self { app }
    }
}
use google_blogger2 as api;

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
