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
        let mut app = App::new("games1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20210304")
            .about("The Google Play games service allows developers to enhance games with social leaderboards, achievements, game state, sign-in with Google, and more.")
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
        let mut achievement_definitions0 = SubCommand::with_name("achievement_definitions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all the achievement definitions for your application.");
            achievement_definitions0 = achievement_definitions0.subcommand(mcmd);
        }
        let mut achievements0 = SubCommand::with_name("achievements")
            .setting(AppSettings::ColoredHelp)
            .about(
                "methods: increment, list, reveal, set_steps_at_least, unlock and update_multiple",
            );
        {
            let mcmd = SubCommand::with_name("increment").about("Increments the steps of the achievement with the given ID for the currently authenticated player.");
            achievements0 = achievements0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the progress for all your application\'s achievements for the currently authenticated player.");
            achievements0 = achievements0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("reveal").about("Sets the state of the achievement with the given ID to `REVEALED` for the currently authenticated player.");
            achievements0 = achievements0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_steps_at_least").about("Sets the steps for the currently authenticated player towards unlocking an achievement. If the steps parameter is less than the current number of steps that the player already gained for the achievement, the achievement is not modified.");
            achievements0 = achievements0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("unlock")
                .about("Unlocks this achievement for the currently authenticated player.");
            achievements0 = achievements0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_multiple")
                .about("Updates multiple achievements for the currently authenticated player.");
            achievements0 = achievements0.subcommand(mcmd);
        }
        let mut applications0 = SubCommand::with_name("applications")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, get_end_point, played and verify");
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves the metadata of the application with the given ID. If the requested application is not available for the specified `platformType`, the returned response will not include any instance data.");
            applications0 = applications0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_end_point")
                .about("Returns a URL for the requested end point type.");
            applications0 = applications0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("played").about(
                "Indicate that the currently authenticated user is playing your application.",
            );
            applications0 = applications0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("verify").about("Verifies the auth token provided with this request is for the application with the specified ID, and returns the ID of the player it was granted for.");
            applications0 = applications0.subcommand(mcmd);
        }
        let mut events0 = SubCommand::with_name("events")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list_by_player, list_definitions and record");
        {
            let mcmd = SubCommand::with_name("list_by_player").about("Returns a list showing the current progress on events in this application for the currently authenticated user.");
            events0 = events0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list_definitions")
                .about("Returns a list of the event definitions in this application.");
            events0 = events0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("record").about("Records a batch of changes to the number of times events have occurred for the currently authenticated user of this application.");
            events0 = events0.subcommand(mcmd);
        }
        let mut leaderboards0 = SubCommand::with_name("leaderboards")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Retrieves the metadata of the leaderboard with the given ID.");
            leaderboards0 = leaderboards0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all the leaderboard metadata for your application.");
            leaderboards0 = leaderboards0.subcommand(mcmd);
        }
        let mut metagame0 = SubCommand::with_name("metagame")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get_metagame_config and list_categories_by_player");
        {
            let mcmd = SubCommand::with_name("get_metagame_config")
                .about("Return the metagame configuration data for the calling application.");
            metagame0 = metagame0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list_categories_by_player").about("List play data aggregated per category for the player corresponding to `playerId`.");
            metagame0 = metagame0.subcommand(mcmd);
        }
        let mut players0 = SubCommand::with_name("players")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves the Player resource with the given ID. To retrieve the player for the currently authenticated user, set `playerId` to `me`.");
            players0 = players0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Get the collection of players for the currently authenticated user.");
            players0 = players0.subcommand(mcmd);
        }
        let mut revisions0 = SubCommand::with_name("revisions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: check");
        {
            let mcmd = SubCommand::with_name("check")
                .about("Checks whether the games client is out of date.");
            revisions0 = revisions0.subcommand(mcmd);
        }
        let mut scores0 = SubCommand::with_name("scores")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, list, list_window, submit and submit_multiple");
        {
            let mcmd = SubCommand::with_name("get").about("Get high scores, and optionally ranks, in leaderboards for the currently authenticated player. For a specific time span, `leaderboardId` can be set to `ALL` to retrieve data for all leaderboards in a given time span. `NOTE: You cannot ask for \'ALL\' leaderboards and \'ALL\' timeSpans in the same request; only one parameter may be set to \'ALL\'.");
            scores0 = scores0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists the scores in a leaderboard, starting from the top.");
            scores0 = scores0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list_window").about(
                "Lists the scores in a leaderboard around (and including) a player\'s score.",
            );
            scores0 = scores0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("submit")
                .about("Submits a score to the specified leaderboard.");
            scores0 = scores0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("submit_multiple")
                .about("Submits multiple scores to leaderboards.");
            scores0 = scores0.subcommand(mcmd);
        }
        let mut snapshots0 = SubCommand::with_name("snapshots")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Retrieves the metadata for a given snapshot ID.");
            snapshots0 = snapshots0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves a list of snapshots created by your application for the player corresponding to the player ID.");
            snapshots0 = snapshots0.subcommand(mcmd);
        }
        let mut snapshots_extended0 = SubCommand::with_name("snapshots_extended")
            .setting(AppSettings::ColoredHelp)
            .about("methods: resolve_snapshot_head");
        {
            let mcmd = SubCommand::with_name("resolve_snapshot_head").about("Resolves any potential conflicts according to the resolution policy specified in the request and returns the snapshot head after the resolution.");
            snapshots_extended0 = snapshots_extended0.subcommand(mcmd);
        }
        let mut stats0 = SubCommand::with_name("stats")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Returns engagement and spend statistics in this application for the currently authenticated user.");
            stats0 = stats0.subcommand(mcmd);
        }
        app = app.subcommand(stats0);
        app = app.subcommand(snapshots_extended0);
        app = app.subcommand(snapshots0);
        app = app.subcommand(scores0);
        app = app.subcommand(revisions0);
        app = app.subcommand(players0);
        app = app.subcommand(metagame0);
        app = app.subcommand(leaderboards0);
        app = app.subcommand(events0);
        app = app.subcommand(applications0);
        app = app.subcommand(achievements0);
        app = app.subcommand(achievement_definitions0);

        Self { app }
    }
}
use google_games1 as api;

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
