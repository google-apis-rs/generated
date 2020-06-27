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
        let mut app = App::new("drive3")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200615")
            .about("Manages files in Drive including uploading, downloading, searching, detecting changes, and updating sharing permissions.")
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
        let mut about0 = SubCommand::with_name("about")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about(
                "Gets information about the user, the user\'s Drive, and system capabilities.",
            );
            about0 = about0.subcommand(mcmd);
        }
        let mut changes0 = SubCommand::with_name("changes")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get_start_page_token, list and watch");
        {
            let mcmd = SubCommand::with_name("get_start_page_token")
                .about("Gets the starting pageToken for listing future changes.");
            changes0 = changes0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists the changes for a user or shared drive.");
            changes0 = changes0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("watch").about("Subscribes to changes for a user.");
            changes0 = changes0.subcommand(mcmd);
        }
        let mut channels0 = SubCommand::with_name("channels")
            .setting(AppSettings::ColoredHelp)
            .about("methods: stop");
        {
            let mcmd =
                SubCommand::with_name("stop").about("Stop watching resources through this channel");
            channels0 = channels0.subcommand(mcmd);
        }
        let mut comments0 = SubCommand::with_name("comments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and update");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new comment on a file.");
            comments0 = comments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a comment.");
            comments0 = comments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a comment by ID.");
            comments0 = comments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists a file\'s comments.");
            comments0 = comments0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("update").about("Updates a comment with patch semantics.");
            comments0 = comments0.subcommand(mcmd);
        }
        let mut drives0 = SubCommand::with_name("drives")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, hide, list, unhide and update");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new shared drive.");
            drives0 = drives0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Permanently deletes a shared drive for which the user is an organizer. The shared drive cannot contain any untrashed items.");
            drives0 = drives0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a shared drive\'s metadata by ID.");
            drives0 = drives0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("hide").about("Hides a shared drive from the default view.");
            drives0 = drives0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the user\'s shared drives.");
            drives0 = drives0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("unhide")
                .about("Restores a shared drive to the default view.");
            drives0 = drives0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("update").about("Updates the metadate for a shared drive.");
            drives0 = drives0.subcommand(mcmd);
        }
        let mut files0 = SubCommand::with_name("files")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: copy, create, delete, empty_trash, export, generate_ids, get, list, update and watch");
        {
            let mcmd = SubCommand::with_name("copy").about(
                "Creates a copy of a file and applies any requested updates with patch semantics.",
            );
            files0 = files0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new file.");
            files0 = files0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Permanently deletes a file owned by the user without moving it to the trash. If the file belongs to a shared drive the user must be an organizer on the parent. If the target is a folder, all descendants owned by the user are also deleted.");
            files0 = files0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("empty_trash")
                .about("Permanently deletes all of the user\'s trashed files.");
            files0 = files0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("export").about("Exports a Google Doc to the requested MIME type and returns the exported content. Please note that the exported content is limited to 10MB.");
            files0 = files0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("generate_ids").about(
                "Generates a set of file IDs which can be provided in create or copy requests.",
            );
            files0 = files0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets a file\'s metadata or content by ID.");
            files0 = files0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists or searches files.");
            files0 = files0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update")
                .about("Updates a file\'s metadata and/or content with patch semantics.");
            files0 = files0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("watch").about("Subscribes to changes to a file");
            files0 = files0.subcommand(mcmd);
        }
        let mut permissions0 = SubCommand::with_name("permissions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and update");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a permission for a file or shared drive.");
            permissions0 = permissions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a permission.");
            permissions0 = permissions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a permission by ID.");
            permissions0 = permissions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists a file\'s or shared drive\'s permissions.");
            permissions0 = permissions0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("update").about("Updates a permission with patch semantics.");
            permissions0 = permissions0.subcommand(mcmd);
        }
        let mut replies0 = SubCommand::with_name("replies")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and update");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new reply to a comment.");
            replies0 = replies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a reply.");
            replies0 = replies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a reply by ID.");
            replies0 = replies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists a comment\'s replies.");
            replies0 = replies0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("update").about("Updates a reply with patch semantics.");
            replies0 = replies0.subcommand(mcmd);
        }
        let mut revisions0 = SubCommand::with_name("revisions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, list and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Permanently deletes a file version. You can only delete revisions for files with binary content in Google Drive, like images or videos. Revisions for other files, like Google Docs or Sheets, and the last remaining file version can\'t be deleted.");
            revisions0 = revisions0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets a revision\'s metadata or content by ID.");
            revisions0 = revisions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists a file\'s revisions.");
            revisions0 = revisions0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("update").about("Updates a revision with patch semantics.");
            revisions0 = revisions0.subcommand(mcmd);
        }
        let mut teamdrives0 = SubCommand::with_name("teamdrives")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and update");
        {
            let mcmd =
                SubCommand::with_name("create").about("Deprecated use drives.create instead.");
            teamdrives0 = teamdrives0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deprecated use drives.delete instead.");
            teamdrives0 = teamdrives0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Deprecated use drives.get instead.");
            teamdrives0 = teamdrives0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Deprecated use drives.list instead.");
            teamdrives0 = teamdrives0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("update").about("Deprecated use drives.update instead");
            teamdrives0 = teamdrives0.subcommand(mcmd);
        }
        app = app.subcommand(teamdrives0);
        app = app.subcommand(revisions0);
        app = app.subcommand(replies0);
        app = app.subcommand(permissions0);
        app = app.subcommand(files0);
        app = app.subcommand(drives0);
        app = app.subcommand(comments0);
        app = app.subcommand(channels0);
        app = app.subcommand(changes0);
        app = app.subcommand(about0);

        Self { app }
    }
}
use google_drive3 as api;

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
