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
        let mut app = App::new("cloudasset1_p1beta1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200613")
            .about("The cloud asset API manages the history and inventory of cloud resources.")
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
        let mut iam_policies0 = SubCommand::with_name("iam_policies")
            .setting(AppSettings::ColoredHelp)
            .about("methods: search_all");
        {
            let mcmd = SubCommand::with_name("search_all").about("Searches all the IAM policies within a given accessible CRM scope\n(project/folder/organization). This RPC gives callers especially\nadministrators the ability to search all the IAM policies within a scope,\neven if they don\'t have `.getIamPolicy` permission of all the IAM policies.\nCallers should have `cloud.assets.SearchAllIamPolicies` permission on the\nrequested scope, otherwise the request will be rejected.");
            iam_policies0 = iam_policies0.subcommand(mcmd);
        }
        let mut resources0 = SubCommand::with_name("resources")
            .setting(AppSettings::ColoredHelp)
            .about("methods: search_all");
        {
            let mcmd = SubCommand::with_name("search_all").about("Searches all the resources within a given accessible CRM scope\n(project/folder/organization). This RPC gives callers especially\nadministrators the ability to search all the resources within a scope, even\nif they don\'t have `.get` permission of all the resources. Callers should\nhave `cloud.assets.SearchAllResources` permission on the requested scope,\notherwise the request will be rejected.");
            resources0 = resources0.subcommand(mcmd);
        }
        app = app.subcommand(resources0);
        app = app.subcommand(iam_policies0);

        Self { app }
    }
}
use google_cloudasset1_p1beta1 as api;

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
