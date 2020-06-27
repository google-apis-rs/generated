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
        let mut app = App::new("clouddebugger2")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200619")
            .about("Examines the call stack and variables of a running application without stopping or slowing it down.\n")
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
        let mut controller0 = SubCommand::with_name("controller")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: debuggees");
        let mut debugger0 = SubCommand::with_name("debugger")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: debuggees");
        let mut debuggees1 = SubCommand::with_name("debuggees")
            .setting(AppSettings::ColoredHelp)
            .about("methods: register");
        {
            let mcmd = SubCommand::with_name("register").about("Registers the debuggee with the controller service.\n\nAll agents attached to the same application must call this method with\nexactly the same request content to get back the same stable `debuggee_id`.\nAgents should call this method again whenever `google.rpc.Code.NOT_FOUND`\nis returned from any controller method.\n\nThis protocol allows the controller service to disable debuggees, recover\nfrom data loss, or change the `debuggee_id` format. Agents must handle\n`debuggee_id` value changing upon re-registration.");
            debuggees1 = debuggees1.subcommand(mcmd);
        }
        let mut debuggees1 = SubCommand::with_name("debuggees")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all the debuggees that the user has access to.");
            debuggees1 = debuggees1.subcommand(mcmd);
        }
        let mut breakpoints2 = SubCommand::with_name("breakpoints")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list and update");
        {
            let mcmd = SubCommand::with_name("list").about("Returns the list of all active breakpoints for the debuggee.\n\nThe breakpoint specification (`location`, `condition`, and `expressions`\nfields) is semantically immutable, although the field values may\nchange. For example, an agent may update the location line number\nto reflect the actual line where the breakpoint was set, but this\ndoesn\'t change the breakpoint semantics.\n\nThis means that an agent does not need to check if a breakpoint has changed\nwhen it encounters the same breakpoint on a successive call.\nMoreover, an agent should remember the breakpoints that are completed\nuntil the controller removes them from the active list to avoid\nsetting those breakpoints again.");
            breakpoints2 = breakpoints2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates the breakpoint state or mutable fields.\nThe entire Breakpoint message must be sent back to the controller service.\n\nUpdates to active breakpoint fields are only allowed if the new value\ndoes not change the breakpoint specification. Updates to the `location`,\n`condition` and `expressions` fields should not alter the breakpoint\nsemantics. These may only make changes such as canonicalizing a value\nor snapping the location to the correct line of code.");
            breakpoints2 = breakpoints2.subcommand(mcmd);
        }
        let mut breakpoints2 = SubCommand::with_name("breakpoints")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, list and set");
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes the breakpoint from the debuggee.");
            breakpoints2 = breakpoints2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets breakpoint information.");
            breakpoints2 = breakpoints2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists all breakpoints for the debuggee.");
            breakpoints2 = breakpoints2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set").about("Sets the breakpoint to the debuggee.");
            breakpoints2 = breakpoints2.subcommand(mcmd);
        }
        debuggees1 = debuggees1.subcommand(breakpoints2);
        debuggees1 = debuggees1.subcommand(breakpoints2);
        debugger0 = debugger0.subcommand(debuggees1);
        controller0 = controller0.subcommand(debuggees1);
        app = app.subcommand(debugger0);
        app = app.subcommand(controller0);

        Self { app }
    }
}
use google_clouddebugger2 as api;

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
