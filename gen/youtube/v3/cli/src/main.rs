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
        let mut app = App::new("youtube3")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20210310")
            .about("The YouTube Data API v3 is an API that provides access to YouTube data, such as videos, playlists, and channels.")
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
        let mut abuse_reports0 = SubCommand::with_name("abuse_reports")
            .setting(AppSettings::ColoredHelp)
            .about("methods: insert");
        {
            let mcmd = SubCommand::with_name("insert")
                .about("Inserts a new resource into this collection.");
            abuse_reports0 = abuse_reports0.subcommand(mcmd);
        }
        let mut activities0 = SubCommand::with_name("activities")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a list of resources, possibly filtered.");
            activities0 = activities0.subcommand(mcmd);
        }
        let mut captions0 = SubCommand::with_name("captions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, download, insert, list and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a resource.");
            captions0 = captions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("download").about("Downloads a caption track.");
            captions0 = captions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert")
                .about("Inserts a new resource into this collection.");
            captions0 = captions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a list of resources, possibly filtered.");
            captions0 = captions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates an existing resource.");
            captions0 = captions0.subcommand(mcmd);
        }
        let mut channel_banners0 = SubCommand::with_name("channel_banners")
            .setting(AppSettings::ColoredHelp)
            .about("methods: insert");
        {
            let mcmd = SubCommand::with_name("insert")
                .about("Inserts a new resource into this collection.");
            channel_banners0 = channel_banners0.subcommand(mcmd);
        }
        let mut channel_sections0 = SubCommand::with_name("channel_sections")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, insert, list and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a resource.");
            channel_sections0 = channel_sections0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert")
                .about("Inserts a new resource into this collection.");
            channel_sections0 = channel_sections0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a list of resources, possibly filtered.");
            channel_sections0 = channel_sections0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates an existing resource.");
            channel_sections0 = channel_sections0.subcommand(mcmd);
        }
        let mut channels0 = SubCommand::with_name("channels")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list and update");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a list of resources, possibly filtered.");
            channels0 = channels0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates an existing resource.");
            channels0 = channels0.subcommand(mcmd);
        }
        let mut comment_threads0 = SubCommand::with_name("comment_threads")
            .setting(AppSettings::ColoredHelp)
            .about("methods: insert, list and update");
        {
            let mcmd = SubCommand::with_name("insert")
                .about("Inserts a new resource into this collection.");
            comment_threads0 = comment_threads0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a list of resources, possibly filtered.");
            comment_threads0 = comment_threads0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates an existing resource.");
            comment_threads0 = comment_threads0.subcommand(mcmd);
        }
        let mut comments0 = SubCommand::with_name("comments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, insert, list, mark_as_spam, set_moderation_status and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a resource.");
            comments0 = comments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert")
                .about("Inserts a new resource into this collection.");
            comments0 = comments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a list of resources, possibly filtered.");
            comments0 = comments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("mark_as_spam").about("Expresses the caller\'s opinion that one or more comments should be flagged as spam.");
            comments0 = comments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_moderation_status")
                .about("Sets the moderation status of one or more comments.");
            comments0 = comments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates an existing resource.");
            comments0 = comments0.subcommand(mcmd);
        }
        let mut i_1_8n_languages0 = SubCommand::with_name("i_1_8n_languages")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a list of resources, possibly filtered.");
            i_1_8n_languages0 = i_1_8n_languages0.subcommand(mcmd);
        }
        let mut i_1_8n_regions0 = SubCommand::with_name("i_1_8n_regions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a list of resources, possibly filtered.");
            i_1_8n_regions0 = i_1_8n_regions0.subcommand(mcmd);
        }
        let mut live_broadcasts0 = SubCommand::with_name("live_broadcasts")
            .setting(AppSettings::ColoredHelp)
            .about("methods: bind, delete, insert, list, transition and update");
        {
            let mcmd = SubCommand::with_name("bind").about("Bind a broadcast to a stream.");
            live_broadcasts0 = live_broadcasts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Delete a given broadcast.");
            live_broadcasts0 = live_broadcasts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert")
                .about("Inserts a new stream for the authenticated user.");
            live_broadcasts0 = live_broadcasts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieve the list of broadcasts associated with the given channel.");
            live_broadcasts0 = live_broadcasts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("transition")
                .about("Transition a broadcast to a given status.");
            live_broadcasts0 = live_broadcasts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update")
                .about("Updates an existing broadcast for the authenticated user.");
            live_broadcasts0 = live_broadcasts0.subcommand(mcmd);
        }
        let mut live_chat_bans0 = SubCommand::with_name("live_chat_bans")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete and insert");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a chat ban.");
            live_chat_bans0 = live_chat_bans0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert")
                .about("Inserts a new resource into this collection.");
            live_chat_bans0 = live_chat_bans0.subcommand(mcmd);
        }
        let mut live_chat_messages0 = SubCommand::with_name("live_chat_messages")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, insert and list");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a chat message.");
            live_chat_messages0 = live_chat_messages0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert")
                .about("Inserts a new resource into this collection.");
            live_chat_messages0 = live_chat_messages0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a list of resources, possibly filtered.");
            live_chat_messages0 = live_chat_messages0.subcommand(mcmd);
        }
        let mut live_chat_moderators0 = SubCommand::with_name("live_chat_moderators")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, insert and list");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a chat moderator.");
            live_chat_moderators0 = live_chat_moderators0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert")
                .about("Inserts a new resource into this collection.");
            live_chat_moderators0 = live_chat_moderators0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a list of resources, possibly filtered.");
            live_chat_moderators0 = live_chat_moderators0.subcommand(mcmd);
        }
        let mut live_streams0 = SubCommand::with_name("live_streams")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, insert, list and update");
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes an existing stream for the authenticated user.");
            live_streams0 = live_streams0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert")
                .about("Inserts a new stream for the authenticated user.");
            live_streams0 = live_streams0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieve the list of streams associated with the given channel. --");
            live_streams0 = live_streams0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update")
                .about("Updates an existing stream for the authenticated user.");
            live_streams0 = live_streams0.subcommand(mcmd);
        }
        let mut members0 = SubCommand::with_name("members")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about(
                "Retrieves a list of members that match the request criteria for a channel.",
            );
            members0 = members0.subcommand(mcmd);
        }
        let mut memberships_levels0 = SubCommand::with_name("memberships_levels")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a list of all pricing levels offered by a creator to the fans.");
            memberships_levels0 = memberships_levels0.subcommand(mcmd);
        }
        let mut playlist_items0 = SubCommand::with_name("playlist_items")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, insert, list and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a resource.");
            playlist_items0 = playlist_items0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert")
                .about("Inserts a new resource into this collection.");
            playlist_items0 = playlist_items0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a list of resources, possibly filtered.");
            playlist_items0 = playlist_items0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates an existing resource.");
            playlist_items0 = playlist_items0.subcommand(mcmd);
        }
        let mut playlists0 = SubCommand::with_name("playlists")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, insert, list and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a resource.");
            playlists0 = playlists0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert")
                .about("Inserts a new resource into this collection.");
            playlists0 = playlists0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a list of resources, possibly filtered.");
            playlists0 = playlists0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates an existing resource.");
            playlists0 = playlists0.subcommand(mcmd);
        }
        let mut search0 = SubCommand::with_name("search")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves a list of search resources");
            search0 = search0.subcommand(mcmd);
        }
        let mut subscriptions0 = SubCommand::with_name("subscriptions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, insert and list");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a resource.");
            subscriptions0 = subscriptions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert")
                .about("Inserts a new resource into this collection.");
            subscriptions0 = subscriptions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a list of resources, possibly filtered.");
            subscriptions0 = subscriptions0.subcommand(mcmd);
        }
        let mut super_chat_events0 = SubCommand::with_name("super_chat_events")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a list of resources, possibly filtered.");
            super_chat_events0 = super_chat_events0.subcommand(mcmd);
        }
        let mut tests0 = SubCommand::with_name("tests")
            .setting(AppSettings::ColoredHelp)
            .about("methods: insert");
        {
            let mcmd = SubCommand::with_name("insert").about("POST method.");
            tests0 = tests0.subcommand(mcmd);
        }
        let mut third_party_links0 = SubCommand::with_name("third_party_links")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, insert, list and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a resource.");
            third_party_links0 = third_party_links0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert")
                .about("Inserts a new resource into this collection.");
            third_party_links0 = third_party_links0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a list of resources, possibly filtered.");
            third_party_links0 = third_party_links0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates an existing resource.");
            third_party_links0 = third_party_links0.subcommand(mcmd);
        }
        let mut thumbnails0 = SubCommand::with_name("thumbnails")
            .setting(AppSettings::ColoredHelp)
            .about("methods: set");
        {
            let mcmd = SubCommand::with_name("set").about("As this is not an insert in a strict sense (it supports uploading/setting of a thumbnail for multiple videos, which doesn\'t result in creation of a single resource), I use a custom verb here.");
            thumbnails0 = thumbnails0.subcommand(mcmd);
        }
        let mut video_abuse_report_reasons0 = SubCommand::with_name("video_abuse_report_reasons")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a list of resources, possibly filtered.");
            video_abuse_report_reasons0 = video_abuse_report_reasons0.subcommand(mcmd);
        }
        let mut video_categories0 = SubCommand::with_name("video_categories")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a list of resources, possibly filtered.");
            video_categories0 = video_categories0.subcommand(mcmd);
        }
        let mut videos0 = SubCommand::with_name("videos")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get_rating, insert, list, rate, report_abuse and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a resource.");
            videos0 = videos0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_rating").about("Retrieves the ratings that the authorized user gave to a list of specified videos.");
            videos0 = videos0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert")
                .about("Inserts a new resource into this collection.");
            videos0 = videos0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a list of resources, possibly filtered.");
            videos0 = videos0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("rate").about(
                "Adds a like or dislike rating to a video or removes a rating from a video.",
            );
            videos0 = videos0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("report_abuse").about("Report abuse for a video.");
            videos0 = videos0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates an existing resource.");
            videos0 = videos0.subcommand(mcmd);
        }
        let mut watermarks0 = SubCommand::with_name("watermarks")
            .setting(AppSettings::ColoredHelp)
            .about("methods: set and unset");
        {
            let mcmd = SubCommand::with_name("set")
                .about("Allows upload of watermark image and setting it for a channel.");
            watermarks0 = watermarks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("unset").about("Allows removal of channel watermark.");
            watermarks0 = watermarks0.subcommand(mcmd);
        }
        app = app.subcommand(watermarks0);
        app = app.subcommand(videos0);
        app = app.subcommand(video_categories0);
        app = app.subcommand(video_abuse_report_reasons0);
        app = app.subcommand(thumbnails0);
        app = app.subcommand(third_party_links0);
        app = app.subcommand(tests0);
        app = app.subcommand(super_chat_events0);
        app = app.subcommand(subscriptions0);
        app = app.subcommand(search0);
        app = app.subcommand(playlists0);
        app = app.subcommand(playlist_items0);
        app = app.subcommand(memberships_levels0);
        app = app.subcommand(members0);
        app = app.subcommand(live_streams0);
        app = app.subcommand(live_chat_moderators0);
        app = app.subcommand(live_chat_messages0);
        app = app.subcommand(live_chat_bans0);
        app = app.subcommand(live_broadcasts0);
        app = app.subcommand(i_1_8n_regions0);
        app = app.subcommand(i_1_8n_languages0);
        app = app.subcommand(comments0);
        app = app.subcommand(comment_threads0);
        app = app.subcommand(channels0);
        app = app.subcommand(channel_sections0);
        app = app.subcommand(channel_banners0);
        app = app.subcommand(captions0);
        app = app.subcommand(activities0);
        app = app.subcommand(abuse_reports0);

        Self { app }
    }
}
use google_youtube3 as api;

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
