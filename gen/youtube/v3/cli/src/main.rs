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
            .version("0.1.0-20190827")
            .about("Supports core YouTube features, such as uploading videos, creating and managing playlists, searching for content, and much more.")
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
        let mut activities0 = SubCommand::with_name("activities")
            .setting(AppSettings::ColoredHelp)
            .about("methods: insert and list");
        {
            let mcmd = SubCommand::with_name("insert").about("Posts a bulletin for a specific channel. (The user submitting the request must be authorized to act on the channel\'s behalf.)\n\nNote: Even though an activity resource can contain information about actions like a user rating a video or marking a video as a favorite, you need to use other API methods to generate those activity resources. For example, you would use the API\'s videos.rate() method to rate a video and the playlistItems.insert() method to mark a video as a favorite.");
            activities0 = activities0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Returns a list of channel activity events that match the request criteria. For example, you can retrieve events associated with a particular channel, events associated with the user\'s subscriptions and Google+ friends, or the YouTube home page feed, which is customized for each user.");
            activities0 = activities0.subcommand(mcmd);
        }
        let mut captions0 = SubCommand::with_name("captions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, download, insert, list and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a specified caption track.");
            captions0 = captions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("download").about("Downloads a caption track. The caption track is returned in its original format unless the request specifies a value for the tfmt parameter and in its original language unless the request specifies a value for the tlang parameter.");
            captions0 = captions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Uploads a caption track.");
            captions0 = captions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Returns a list of caption tracks that are associated with a specified video. Note that the API response does not contain the actual captions and that the captions.download method provides the ability to retrieve a caption track.");
            captions0 = captions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a caption track. When updating a caption track, you can change the track\'s draft status, upload a new caption file for the track, or both.");
            captions0 = captions0.subcommand(mcmd);
        }
        let mut channel_banners0 = SubCommand::with_name("channel_banners")
            .setting(AppSettings::ColoredHelp)
            .about("methods: insert");
        {
            let mcmd = SubCommand::with_name("insert").about("Uploads a channel banner image to YouTube. This method represents the first two steps in a three-step process to update the banner image for a channel:\n\n- Call the channelBanners.insert method to upload the binary image data to YouTube. The image must have a 16:9 aspect ratio and be at least 2120x1192 pixels.\n- Extract the url property\'s value from the response that the API returns for step 1.\n- Call the channels.update method to update the channel\'s branding settings. Set the brandingSettings.image.bannerExternalUrl property\'s value to the URL obtained in step 2.");
            channel_banners0 = channel_banners0.subcommand(mcmd);
        }
        let mut channel_sections0 = SubCommand::with_name("channel_sections")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, insert, list and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a channelSection.");
            channel_sections0 = channel_sections0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert")
                .about("Adds a channelSection for the authenticated user\'s channel.");
            channel_sections0 = channel_sections0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns channelSection resources that match the API request criteria.");
            channel_sections0 = channel_sections0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Update a channelSection.");
            channel_sections0 = channel_sections0.subcommand(mcmd);
        }
        let mut channels0 = SubCommand::with_name("channels")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list and update");
        {
            let mcmd = SubCommand::with_name("list").about("Returns a collection of zero or more channel resources that match the request criteria.");
            channels0 = channels0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a channel\'s metadata. Note that this method currently only supports updates to the channel resource\'s brandingSettings and invideoPromotion objects and their child properties.");
            channels0 = channels0.subcommand(mcmd);
        }
        let mut comment_threads0 = SubCommand::with_name("comment_threads")
            .setting(AppSettings::ColoredHelp)
            .about("methods: insert, list and update");
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a new top-level comment. To add a reply to an existing comment, use the comments.insert method instead.");
            comment_threads0 = comment_threads0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns a list of comment threads that match the API request parameters.");
            comment_threads0 = comment_threads0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update")
                .about("Modifies the top-level comment in a comment thread.");
            comment_threads0 = comment_threads0.subcommand(mcmd);
        }
        let mut comments0 = SubCommand::with_name("comments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, insert, list, mark_as_spam, set_moderation_status and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a comment.");
            comments0 = comments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a reply to an existing comment. Note: To create a top-level comment, use the commentThreads.insert method.");
            comments0 = comments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns a list of comments that match the API request parameters.");
            comments0 = comments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("mark_as_spam").about("Expresses the caller\'s opinion that one or more comments should be flagged as spam.");
            comments0 = comments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_moderation_status").about("Sets the moderation status of one or more comments. The API request must be authorized by the owner of the channel or video associated with the comments.");
            comments0 = comments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Modifies a comment.");
            comments0 = comments0.subcommand(mcmd);
        }
        let mut guide_categories0 = SubCommand::with_name("guide_categories")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about(
                "Returns a list of categories that can be associated with YouTube channels.",
            );
            guide_categories0 = guide_categories0.subcommand(mcmd);
        }
        let mut i_1_8n_languages0 = SubCommand::with_name("i_1_8n_languages")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about(
                "Returns a list of application languages that the YouTube website supports.",
            );
            i_1_8n_languages0 = i_1_8n_languages0.subcommand(mcmd);
        }
        let mut i_1_8n_regions0 = SubCommand::with_name("i_1_8n_regions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns a list of content regions that the YouTube website supports.");
            i_1_8n_regions0 = i_1_8n_regions0.subcommand(mcmd);
        }
        let mut live_broadcasts0 = SubCommand::with_name("live_broadcasts")
            .setting(AppSettings::ColoredHelp)
            .about("methods: bind, control, delete, insert, list, transition and update");
        {
            let mcmd = SubCommand::with_name("bind").about("Binds a YouTube broadcast to a stream or removes an existing binding between a broadcast and a stream. A broadcast can only be bound to one video stream, though a video stream may be bound to more than one broadcast.");
            live_broadcasts0 = live_broadcasts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("control").about(
                "Controls the settings for a slate that can be displayed in the broadcast stream.",
            );
            live_broadcasts0 = live_broadcasts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a broadcast.");
            live_broadcasts0 = live_broadcasts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a broadcast.");
            live_broadcasts0 = live_broadcasts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Returns a list of YouTube broadcasts that match the API request parameters.",
            );
            live_broadcasts0 = live_broadcasts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("transition").about("Changes the status of a YouTube live broadcast and initiates any processes associated with the new status. For example, when you transition a broadcast\'s status to testing, YouTube starts to transmit video to that broadcast\'s monitor stream. Before calling this method, you should confirm that the value of the status.streamStatus property for the stream bound to your broadcast is active.");
            live_broadcasts0 = live_broadcasts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a broadcast. For example, you could modify the broadcast settings defined in the liveBroadcast resource\'s contentDetails object.");
            live_broadcasts0 = live_broadcasts0.subcommand(mcmd);
        }
        let mut live_chat_bans0 = SubCommand::with_name("live_chat_bans")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete and insert");
        {
            let mcmd = SubCommand::with_name("delete").about("Removes a chat ban.");
            live_chat_bans0 = live_chat_bans0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Adds a new ban to the chat.");
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
            let mcmd = SubCommand::with_name("insert").about("Adds a message to a live chat.");
            live_chat_messages0 = live_chat_messages0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists live chat messages for a specific chat.");
            live_chat_messages0 = live_chat_messages0.subcommand(mcmd);
        }
        let mut live_chat_moderators0 = SubCommand::with_name("live_chat_moderators")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, insert and list");
        {
            let mcmd = SubCommand::with_name("delete").about("Removes a chat moderator.");
            live_chat_moderators0 = live_chat_moderators0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Adds a new moderator for the chat.");
            live_chat_moderators0 = live_chat_moderators0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists moderators for a live chat.");
            live_chat_moderators0 = live_chat_moderators0.subcommand(mcmd);
        }
        let mut live_streams0 = SubCommand::with_name("live_streams")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, insert, list and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a video stream.");
            live_streams0 = live_streams0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a video stream. The stream enables you to send your video to YouTube, which can then broadcast the video to your audience.");
            live_streams0 = live_streams0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns a list of video streams that match the API request parameters.");
            live_streams0 = live_streams0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a video stream. If the properties that you want to change cannot be updated, then you need to create a new stream with the proper settings.");
            live_streams0 = live_streams0.subcommand(mcmd);
        }
        let mut members0 = SubCommand::with_name("members")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists members for a channel.");
            members0 = members0.subcommand(mcmd);
        }
        let mut memberships_levels0 = SubCommand::with_name("memberships_levels")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists pricing levels for a channel.");
            memberships_levels0 = memberships_levels0.subcommand(mcmd);
        }
        let mut playlist_items0 = SubCommand::with_name("playlist_items")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, insert, list and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a playlist item.");
            playlist_items0 = playlist_items0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Adds a resource to a playlist.");
            playlist_items0 = playlist_items0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Returns a collection of playlist items that match the API request parameters. You can retrieve all of the playlist items in a specified playlist or retrieve one or more playlist items by their unique IDs.");
            playlist_items0 = playlist_items0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Modifies a playlist item. For example, you could update the item\'s position in the playlist.");
            playlist_items0 = playlist_items0.subcommand(mcmd);
        }
        let mut playlists0 = SubCommand::with_name("playlists")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, insert, list and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a playlist.");
            playlists0 = playlists0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a playlist.");
            playlists0 = playlists0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Returns a collection of playlists that match the API request parameters. For example, you can retrieve all playlists that the authenticated user owns, or you can retrieve one or more playlists by their unique IDs.");
            playlists0 = playlists0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Modifies a playlist. For example, you could change a playlist\'s title, description, or privacy status.");
            playlists0 = playlists0.subcommand(mcmd);
        }
        let mut search0 = SubCommand::with_name("search")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Returns a collection of search results that match the query parameters specified in the API request. By default, a search result set identifies matching video, channel, and playlist resources, but you can also configure queries to only retrieve a specific type of resource.");
            search0 = search0.subcommand(mcmd);
        }
        let mut sponsors0 = SubCommand::with_name("sponsors")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists sponsors for a channel.");
            sponsors0 = sponsors0.subcommand(mcmd);
        }
        let mut subscriptions0 = SubCommand::with_name("subscriptions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, insert and list");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a subscription.");
            subscriptions0 = subscriptions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert")
                .about("Adds a subscription for the authenticated user\'s channel.");
            subscriptions0 = subscriptions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns subscription resources that match the API request criteria.");
            subscriptions0 = subscriptions0.subcommand(mcmd);
        }
        let mut super_chat_events0 = SubCommand::with_name("super_chat_events")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists Super Chat events for a channel.");
            super_chat_events0 = super_chat_events0.subcommand(mcmd);
        }
        let mut thumbnails0 = SubCommand::with_name("thumbnails")
            .setting(AppSettings::ColoredHelp)
            .about("methods: set");
        {
            let mcmd = SubCommand::with_name("set")
                .about("Uploads a custom video thumbnail to YouTube and sets it for a video.");
            thumbnails0 = thumbnails0.subcommand(mcmd);
        }
        let mut video_abuse_report_reasons0 = SubCommand::with_name("video_abuse_report_reasons")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about(
                "Returns a list of abuse reasons that can be used for reporting abusive videos.",
            );
            video_abuse_report_reasons0 = video_abuse_report_reasons0.subcommand(mcmd);
        }
        let mut video_categories0 = SubCommand::with_name("video_categories")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns a list of categories that can be associated with YouTube videos.");
            video_categories0 = video_categories0.subcommand(mcmd);
        }
        let mut videos0 = SubCommand::with_name("videos")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get_rating, insert, list, rate, report_abuse and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a YouTube video.");
            videos0 = videos0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_rating").about("Retrieves the ratings that the authorized user gave to a list of specified videos.");
            videos0 = videos0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert")
                .about("Uploads a video to YouTube and optionally sets the video\'s metadata.");
            videos0 = videos0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns a list of videos that match the API request parameters.");
            videos0 = videos0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("rate")
                .about("Add a like or dislike rating to a video or remove a rating from a video.");
            videos0 = videos0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("report_abuse").about("Report abuse for a video.");
            videos0 = videos0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a video\'s metadata.");
            videos0 = videos0.subcommand(mcmd);
        }
        let mut watermarks0 = SubCommand::with_name("watermarks")
            .setting(AppSettings::ColoredHelp)
            .about("methods: set and unset");
        {
            let mcmd = SubCommand::with_name("set")
                .about("Uploads a watermark image to YouTube and sets it for a channel.");
            watermarks0 = watermarks0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("unset").about("Deletes a channel\'s watermark image.");
            watermarks0 = watermarks0.subcommand(mcmd);
        }
        app = app.subcommand(watermarks0);
        app = app.subcommand(videos0);
        app = app.subcommand(video_categories0);
        app = app.subcommand(video_abuse_report_reasons0);
        app = app.subcommand(thumbnails0);
        app = app.subcommand(super_chat_events0);
        app = app.subcommand(subscriptions0);
        app = app.subcommand(sponsors0);
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
        app = app.subcommand(guide_categories0);
        app = app.subcommand(comments0);
        app = app.subcommand(comment_threads0);
        app = app.subcommand(channels0);
        app = app.subcommand(channel_sections0);
        app = app.subcommand(channel_banners0);
        app = app.subcommand(captions0);
        app = app.subcommand(activities0);

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
