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
        let mut app = App::new("admin1_directory")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200605")
            .about("Manages enterprise resources such as users and groups, administrative notifications, security features, and more.")
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
        let mut asps0 = SubCommand::with_name("asps")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get and list");
        {
            let mcmd = SubCommand::with_name("delete").about("Delete an ASP issued by a user.");
            asps0 = asps0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Get information about an ASP issued by a user.");
            asps0 = asps0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List the ASPs issued by a user.");
            asps0 = asps0.subcommand(mcmd);
        }
        let mut channels0 = SubCommand::with_name("channels")
            .setting(AppSettings::ColoredHelp)
            .about("methods: stop");
        {
            let mcmd =
                SubCommand::with_name("stop").about("Stop watching resources through this channel");
            channels0 = channels0.subcommand(mcmd);
        }
        let mut chromeosdevices0 = SubCommand::with_name("chromeosdevices")
            .setting(AppSettings::ColoredHelp)
            .about("methods: action, get, list, move_devices_to_ou, patch and update");
        {
            let mcmd = SubCommand::with_name("action").about("Take action on Chrome OS Device");
            chromeosdevices0 = chromeosdevices0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieve Chrome OS Device");
            chromeosdevices0 = chromeosdevices0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieve all Chrome OS Devices of a customer (paginated)");
            chromeosdevices0 = chromeosdevices0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("move_devices_to_ou")
                .about("Move or insert multiple Chrome OS Devices to organizational unit");
            chromeosdevices0 = chromeosdevices0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Update Chrome OS Device. This method supports patch semantics.");
            chromeosdevices0 = chromeosdevices0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Update Chrome OS Device");
            chromeosdevices0 = chromeosdevices0.subcommand(mcmd);
        }
        let mut customers0 = SubCommand::with_name("customers")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, patch and update");
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves a customer.");
            customers0 = customers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates a customer. This method supports patch semantics.");
            customers0 = customers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a customer.");
            customers0 = customers0.subcommand(mcmd);
        }
        let mut domain_aliases0 = SubCommand::with_name("domain_aliases")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert and list");
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes a Domain Alias of the customer.");
            domain_aliases0 = domain_aliases0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Retrieves a domain alias of the customer.");
            domain_aliases0 = domain_aliases0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("insert").about("Inserts a Domain alias of the customer.");
            domain_aliases0 = domain_aliases0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists the domain aliases of the customer.");
            domain_aliases0 = domain_aliases0.subcommand(mcmd);
        }
        let mut domains0 = SubCommand::with_name("domains")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert and list");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a domain of the customer.");
            domains0 = domains0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves a domain of the customer.");
            domains0 = domains0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Inserts a domain of the customer.");
            domains0 = domains0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the domains of the customer.");
            domains0 = domains0.subcommand(mcmd);
        }
        let mut groups0 = SubCommand::with_name("groups")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Delete Group");
            groups0 = groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieve Group");
            groups0 = groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Create Group");
            groups0 = groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieve all groups of a domain or of a user given a userKey (paginated)");
            groups0 = groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Update Group. This method supports patch semantics.");
            groups0 = groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Update Group");
            groups0 = groups0.subcommand(mcmd);
        }
        let mut members0 = SubCommand::with_name("members")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, has_member, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Remove membership.");
            members0 = members0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieve Group Member");
            members0 = members0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("has_member").about("Checks whether the given user is a member of the group. Membership can be direct or nested.");
            members0 = members0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Add user to the specified group.");
            members0 = members0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Retrieve all members in a group (paginated)");
            members0 = members0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Update membership of a user in the specified group. This method supports patch semantics.");
            members0 = members0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update")
                .about("Update membership of a user in the specified group.");
            members0 = members0.subcommand(mcmd);
        }
        let mut mobiledevices0 = SubCommand::with_name("mobiledevices")
            .setting(AppSettings::ColoredHelp)
            .about("methods: action, delete, get and list");
        {
            let mcmd = SubCommand::with_name("action").about("Take action on Mobile Device");
            mobiledevices0 = mobiledevices0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Delete Mobile Device");
            mobiledevices0 = mobiledevices0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieve Mobile Device");
            mobiledevices0 = mobiledevices0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieve all Mobile Devices of a customer (paginated)");
            mobiledevices0 = mobiledevices0.subcommand(mcmd);
        }
        let mut notifications0 = SubCommand::with_name("notifications")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, list, patch and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a notification");
            notifications0 = notifications0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves a notification.");
            notifications0 = notifications0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves a list of notifications.");
            notifications0 = notifications0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates a notification. This method supports patch semantics.");
            notifications0 = notifications0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a notification.");
            notifications0 = notifications0.subcommand(mcmd);
        }
        let mut orgunits0 = SubCommand::with_name("orgunits")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Remove organizational unit");
            orgunits0 = orgunits0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieve organizational unit");
            orgunits0 = orgunits0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Add organizational unit");
            orgunits0 = orgunits0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieve all organizational units");
            orgunits0 = orgunits0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Update organizational unit. This method supports patch semantics.");
            orgunits0 = orgunits0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Update organizational unit");
            orgunits0 = orgunits0.subcommand(mcmd);
        }
        let mut privileges0 = SubCommand::with_name("privileges")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a paginated list of all privileges for a customer.");
            privileges0 = privileges0.subcommand(mcmd);
        }
        let mut resources0 = SubCommand::with_name("resources")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: buildings, calendars and features");
        let mut role_assignments0 = SubCommand::with_name("role_assignments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert and list");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a role assignment.");
            role_assignments0 = role_assignments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieve a role assignment.");
            role_assignments0 = role_assignments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a role assignment.");
            role_assignments0 = role_assignments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a paginated list of all roleAssignments.");
            role_assignments0 = role_assignments0.subcommand(mcmd);
        }
        let mut roles0 = SubCommand::with_name("roles")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a role.");
            roles0 = roles0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves a role.");
            roles0 = roles0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a role.");
            roles0 = roles0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a paginated list of all the roles in a domain.");
            roles0 = roles0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates a role. This method supports patch semantics.");
            roles0 = roles0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a role.");
            roles0 = roles0.subcommand(mcmd);
        }
        let mut schemas0 = SubCommand::with_name("schemas")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Delete schema");
            schemas0 = schemas0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieve schema");
            schemas0 = schemas0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Create schema.");
            schemas0 = schemas0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieve all schemas for a customer");
            schemas0 = schemas0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Update schema. This method supports patch semantics.");
            schemas0 = schemas0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Update schema");
            schemas0 = schemas0.subcommand(mcmd);
        }
        let mut tokens0 = SubCommand::with_name("tokens")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get and list");
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Delete all access tokens issued by a user for an application.");
            tokens0 = tokens0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Get information about an access token issued by a user.");
            tokens0 = tokens0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Returns the set of tokens specified user has issued to 3rd party applications.",
            );
            tokens0 = tokens0.subcommand(mcmd);
        }
        let mut users0 = SubCommand::with_name("users")
            .setting(AppSettings::ColoredHelp)
            .about(
                "methods: delete, get, insert, list, make_admin, patch, undelete, update and watch",
            );
        {
            let mcmd = SubCommand::with_name("delete").about("Delete user");
            users0 = users0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("retrieve user");
            users0 = users0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("create user.");
            users0 = users0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieve either deleted users or all users in a domain (paginated)");
            users0 = users0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("make_admin").about("change admin status of a user");
            users0 = users0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("update user. This method supports patch semantics.");
            users0 = users0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("undelete").about("Undelete a deleted user");
            users0 = users0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("update user");
            users0 = users0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("watch").about("Watch for changes in users list");
            users0 = users0.subcommand(mcmd);
        }
        let mut verification_codes0 = SubCommand::with_name("verification_codes")
            .setting(AppSettings::ColoredHelp)
            .about("methods: generate, invalidate and list");
        {
            let mcmd = SubCommand::with_name("generate")
                .about("Generate new backup verification codes for the user.");
            verification_codes0 = verification_codes0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("invalidate")
                .about("Invalidate the current backup verification codes for the user.");
            verification_codes0 = verification_codes0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Returns the current set of valid backup verification codes for the specified user.");
            verification_codes0 = verification_codes0.subcommand(mcmd);
        }
        let mut aliases1 = SubCommand::with_name("aliases")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, insert and list");
        {
            let mcmd = SubCommand::with_name("delete").about("Remove a alias for the group");
            aliases1 = aliases1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Add a alias for the group");
            aliases1 = aliases1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List all aliases for a group");
            aliases1 = aliases1.subcommand(mcmd);
        }
        let mut buildings1 = SubCommand::with_name("buildings")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a building.");
            buildings1 = buildings1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves a building.");
            buildings1 = buildings1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Inserts a building.");
            buildings1 = buildings1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a list of buildings for an account.");
            buildings1 = buildings1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates a building. This method supports patch semantics.");
            buildings1 = buildings1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a building.");
            buildings1 = buildings1.subcommand(mcmd);
        }
        let mut calendars1 = SubCommand::with_name("calendars")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a calendar resource.");
            calendars1 = calendars1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves a calendar resource.");
            calendars1 = calendars1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Inserts a calendar resource.");
            calendars1 = calendars1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a list of calendar resources for an account.");
            calendars1 = calendars1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a calendar resource.\n\nThis method supports patch semantics, meaning you only need to include the fields you wish to update. Fields that are not present in the request will be preserved. This method supports patch semantics.");
            calendars1 = calendars1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a calendar resource.\n\nThis method supports patch semantics, meaning you only need to include the fields you wish to update. Fields that are not present in the request will be preserved.");
            calendars1 = calendars1.subcommand(mcmd);
        }
        let mut features1 = SubCommand::with_name("features")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, patch, rename and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a feature.");
            features1 = features1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves a feature.");
            features1 = features1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Inserts a feature.");
            features1 = features1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Retrieves a list of features for an account.");
            features1 = features1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates a feature. This method supports patch semantics.");
            features1 = features1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("rename").about("Renames a feature.");
            features1 = features1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a feature.");
            features1 = features1.subcommand(mcmd);
        }
        let mut aliases1 = SubCommand::with_name("aliases")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, insert, list and watch");
        {
            let mcmd = SubCommand::with_name("delete").about("Remove a alias for the user");
            aliases1 = aliases1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Add a alias for the user");
            aliases1 = aliases1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List all aliases for a user");
            aliases1 = aliases1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("watch").about("Watch for changes in user aliases list");
            aliases1 = aliases1.subcommand(mcmd);
        }
        let mut photos1 = SubCommand::with_name("photos")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, patch and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Remove photos for the user");
            photos1 = photos1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieve photo of a user");
            photos1 = photos1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Add a photo for the user. This method supports patch semantics.");
            photos1 = photos1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Add a photo for the user");
            photos1 = photos1.subcommand(mcmd);
        }
        users0 = users0.subcommand(photos1);
        users0 = users0.subcommand(aliases1);
        resources0 = resources0.subcommand(features1);
        resources0 = resources0.subcommand(calendars1);
        resources0 = resources0.subcommand(buildings1);
        groups0 = groups0.subcommand(aliases1);
        app = app.subcommand(verification_codes0);
        app = app.subcommand(users0);
        app = app.subcommand(tokens0);
        app = app.subcommand(schemas0);
        app = app.subcommand(roles0);
        app = app.subcommand(role_assignments0);
        app = app.subcommand(resources0);
        app = app.subcommand(privileges0);
        app = app.subcommand(orgunits0);
        app = app.subcommand(notifications0);
        app = app.subcommand(mobiledevices0);
        app = app.subcommand(members0);
        app = app.subcommand(groups0);
        app = app.subcommand(domains0);
        app = app.subcommand(domain_aliases0);
        app = app.subcommand(customers0);
        app = app.subcommand(chromeosdevices0);
        app = app.subcommand(channels0);
        app = app.subcommand(asps0);

        Self { app }
    }
}
use google_admin1_directory as api;

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
