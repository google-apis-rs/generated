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
        let mut app = App::new("realtimebidding1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20210318")
            .about("Allows external bidders to manage their RTB integration with Google. This includes managing bidder endpoints, QPS quotas, configuring what ad inventory to receive via pretargeting, submitting creatives for verification, and accessing creative metadata such as approval status.")
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
        let mut bidders0 = SubCommand::with_name("bidders")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets a bidder account by its name.");
            bidders0 = bidders0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all the bidder accounts that belong to the caller.");
            bidders0 = bidders0.subcommand(mcmd);
        }
        let mut buyers0 = SubCommand::with_name("buyers")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, get_remarketing_tag and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets a buyer account by its name.");
            buyers0 = buyers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_remarketing_tag").about("Gets remarketing tag for a buyer. A remarketing tag is a piece of JavaScript code that can be placed on a web page. When a user visits a page containing a remarketing tag, Google adds the user to a user list.");
            buyers0 = buyers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all buyer account information the calling buyer user or service account is permissioned to manage.");
            buyers0 = buyers0.subcommand(mcmd);
        }
        let mut creatives1 = SubCommand::with_name("creatives")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list and watch");
        {
            let mcmd = SubCommand::with_name("list").about("Lists creatives.");
            creatives1 = creatives1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("watch").about("Watches all creatives pertaining to a bidder. It is sufficient to invoke this endpoint once per bidder. A Pub/Sub topic will be created and notifications will be pushed to the topic when any of the bidder\'s creatives change status. All of the bidder\'s service accounts will have access to read from the topic. Subsequent invocations of this method will return the existing Pub/Sub configuration.");
            creatives1 = creatives1.subcommand(mcmd);
        }
        let mut endpoints1 = SubCommand::with_name("endpoints")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets a bidder endpoint by its name.");
            endpoints1 = endpoints1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all the bidder\'s endpoints.");
            endpoints1 = endpoints1.subcommand(mcmd);
        }
        let mut pretargeting_configs1 = SubCommand::with_name("pretargeting_configs")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: activate, add_targeted_apps, add_targeted_publishers, add_targeted_sites, create, delete, get, list, patch, remove_targeted_apps, remove_targeted_publishers, remove_targeted_sites and suspend");
        {
            let mcmd =
                SubCommand::with_name("activate").about("Activates a pretargeting configuration.");
            pretargeting_configs1 = pretargeting_configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("add_targeted_apps")
                .about("Adds targeted apps to the pretargeting configuration.");
            pretargeting_configs1 = pretargeting_configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("add_targeted_publishers")
                .about("Adds targeted publishers to the pretargeting config.");
            pretargeting_configs1 = pretargeting_configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("add_targeted_sites")
                .about("Adds targeted sites to the pretargeting configuration.");
            pretargeting_configs1 = pretargeting_configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a pretargeting configuration. A pretargeting configuration\'s state (PretargetingConfig.state) is active upon creation, and it will start to affect traffic shortly after. A bidder may create a maximum of 10 pretargeting configurations. Attempts to exceed this maximum results in a 400 bad request error.");
            pretargeting_configs1 = pretargeting_configs1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes a pretargeting configuration.");
            pretargeting_configs1 = pretargeting_configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a pretargeting configuration.");
            pretargeting_configs1 = pretargeting_configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all pretargeting configurations for a single bidder.");
            pretargeting_configs1 = pretargeting_configs1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("patch").about("Updates a pretargeting configuration.");
            pretargeting_configs1 = pretargeting_configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("remove_targeted_apps")
                .about("Removes targeted apps from the pretargeting configuration.");
            pretargeting_configs1 = pretargeting_configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("remove_targeted_publishers")
                .about("Removes targeted publishers from the pretargeting config.");
            pretargeting_configs1 = pretargeting_configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("remove_targeted_sites")
                .about("Removes targeted sites from the pretargeting configuration.");
            pretargeting_configs1 = pretargeting_configs1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("suspend").about("Suspends a pretargeting configuration.");
            pretargeting_configs1 = pretargeting_configs1.subcommand(mcmd);
        }
        let mut creatives1 = SubCommand::with_name("creatives")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a creative.");
            creatives1 = creatives1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a creative.");
            creatives1 = creatives1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists creatives.");
            creatives1 = creatives1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a creative.");
            creatives1 = creatives1.subcommand(mcmd);
        }
        let mut user_lists1 = SubCommand::with_name("user_lists")
            .setting(AppSettings::ColoredHelp)
            .about("methods: close, create, get, get_remarketing_tag, list, open and update");
        {
            let mcmd = SubCommand::with_name("close").about("Change the status of a user list to CLOSED. This prevents new users from being added to the user list.");
            user_lists1 = user_lists1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Create a new user list.");
            user_lists1 = user_lists1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a user list by its name.");
            user_lists1 = user_lists1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_remarketing_tag").about("Gets remarketing tag for a buyer. A remarketing tag is a piece of JavaScript code that can be placed on a web page. When a user visits a page containing a remarketing tag, Google adds the user to a user list.");
            user_lists1 = user_lists1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists the user lists visible to the current user.");
            user_lists1 = user_lists1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("open").about("Change the status of a user list to OPEN. This allows new users to be added to the user list.");
            user_lists1 = user_lists1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about(
                "Update the given user list. Only user lists with URLRestrictions can be updated.",
            );
            user_lists1 = user_lists1.subcommand(mcmd);
        }
        buyers0 = buyers0.subcommand(user_lists1);
        buyers0 = buyers0.subcommand(creatives1);
        bidders0 = bidders0.subcommand(pretargeting_configs1);
        bidders0 = bidders0.subcommand(endpoints1);
        bidders0 = bidders0.subcommand(creatives1);
        app = app.subcommand(buyers0);
        app = app.subcommand(bidders0);

        Self { app }
    }
}
use google_realtimebidding1 as api;

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
