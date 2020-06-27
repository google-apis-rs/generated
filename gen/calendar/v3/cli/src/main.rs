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
        let mut app = App::new("calendar3")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200621")
            .about("Manipulates events and other calendar data.")
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
        let mut acl0 = SubCommand::with_name("acl")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, patch, update and watch");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an access control rule.");
            acl0 = acl0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns an access control rule.");
            acl0 = acl0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates an access control rule.");
            acl0 = acl0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the rules in the access control list for the calendar.");
            acl0 = acl0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates an access control rule. This method supports patch semantics.");
            acl0 = acl0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates an access control rule.");
            acl0 = acl0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("watch").about("Watch for changes to ACL resources.");
            acl0 = acl0.subcommand(mcmd);
        }
        let mut calendar_list0 = SubCommand::with_name("calendar_list")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, patch, update and watch");
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Removes a calendar from the user\'s calendar list.");
            calendar_list0 = calendar_list0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Returns a calendar from the user\'s calendar list.");
            calendar_list0 = calendar_list0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert")
                .about("Inserts an existing calendar into the user\'s calendar list.");
            calendar_list0 = calendar_list0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the calendars on the user\'s calendar list.");
            calendar_list0 = calendar_list0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an existing calendar on the user\'s calendar list. This method supports patch semantics.");
            calendar_list0 = calendar_list0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update")
                .about("Updates an existing calendar on the user\'s calendar list.");
            calendar_list0 = calendar_list0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("watch")
                .about("Watch for changes to CalendarList resources.");
            calendar_list0 = calendar_list0.subcommand(mcmd);
        }
        let mut calendars0 = SubCommand::with_name("calendars")
            .setting(AppSettings::ColoredHelp)
            .about("methods: clear, delete, get, insert, patch and update");
        {
            let mcmd = SubCommand::with_name("clear").about("Clears a primary calendar. This operation deletes all events associated with the primary calendar of an account.");
            calendars0 = calendars0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a secondary calendar. Use calendars.clear for clearing all events on primary calendars.");
            calendars0 = calendars0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns metadata for a calendar.");
            calendars0 = calendars0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a secondary calendar.");
            calendars0 = calendars0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates metadata for a calendar. This method supports patch semantics.");
            calendars0 = calendars0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates metadata for a calendar.");
            calendars0 = calendars0.subcommand(mcmd);
        }
        let mut channels0 = SubCommand::with_name("channels")
            .setting(AppSettings::ColoredHelp)
            .about("methods: stop");
        {
            let mcmd =
                SubCommand::with_name("stop").about("Stop watching resources through this channel");
            channels0 = channels0.subcommand(mcmd);
        }
        let mut colors0 = SubCommand::with_name("colors")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Returns the color definitions for calendars and events.");
            colors0 = colors0.subcommand(mcmd);
        }
        let mut events0 = SubCommand::with_name("events")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: delete, get, import, insert, instances, list, r#move, patch, quick_add, update and watch");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an event.");
            events0 = events0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns an event.");
            events0 = events0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("import").about("Imports an event. This operation is used to add a private copy of an existing event to a calendar.");
            events0 = events0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates an event.");
            events0 = events0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("instances")
                .about("Returns instances of the specified recurring event.");
            events0 = events0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Returns events on the specified calendar.");
            events0 = events0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("r#move")
                .about("Moves an event to another calendar, i.e. changes an event\'s organizer.");
            events0 = events0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates an event. This method supports patch semantics.");
            events0 = events0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("quick_add")
                .about("Creates an event based on a simple text string.");
            events0 = events0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates an event.");
            events0 = events0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("watch").about("Watch for changes to Events resources.");
            events0 = events0.subcommand(mcmd);
        }
        let mut freebusy0 = SubCommand::with_name("freebusy")
            .setting(AppSettings::ColoredHelp)
            .about("methods: query");
        {
            let mcmd = SubCommand::with_name("query")
                .about("Returns free/busy information for a set of calendars.");
            freebusy0 = freebusy0.subcommand(mcmd);
        }
        let mut settings0 = SubCommand::with_name("settings")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, list and watch");
        {
            let mcmd = SubCommand::with_name("get").about("Returns a single user setting.");
            settings0 = settings0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns all user settings for the authenticated user.");
            settings0 = settings0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("watch").about("Watch for changes to Settings resources.");
            settings0 = settings0.subcommand(mcmd);
        }
        app = app.subcommand(settings0);
        app = app.subcommand(freebusy0);
        app = app.subcommand(events0);
        app = app.subcommand(colors0);
        app = app.subcommand(channels0);
        app = app.subcommand(calendars0);
        app = app.subcommand(calendar_list0);
        app = app.subcommand(acl0);

        Self { app }
    }
}
use google_calendar3 as api;

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
