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
        let mut app = App::new("pagespeedonline5")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200612")
            .about("The PageSpeed Insights API lets you analyze the performance of your website with a simple API.  It offers tailored suggestions for how you can optimize your site, and lets you easily integrate PageSpeed Insights analysis into your development tools and workflow.\n")
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
        let mut pagespeedapi0 = SubCommand::with_name("pagespeedapi")
            .setting(AppSettings::ColoredHelp)
            .about("methods: runpagespeed");
        {
            let mcmd = SubCommand::with_name("runpagespeed").about("Runs PageSpeed analysis on the page at the specified URL, and returns\nPageSpeed scores, a list of suggestions to make that page faster, and other\ninformation.");
            pagespeedapi0 = pagespeedapi0.subcommand(mcmd);
        }
        app = app.subcommand(pagespeedapi0);

        Self { app }
    }
}
use google_pagespeedonline5 as api;

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
