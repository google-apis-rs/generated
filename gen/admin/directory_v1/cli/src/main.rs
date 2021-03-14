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
            .version("0.1.0-20210309")
            .about("Admin SDK lets administrators of enterprise domains to view and manage resources like user, groups etc. It also provides audit and usage reports of domain.")
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
            let mcmd = SubCommand::with_name("stop")
                .about("Stop watching resources through this channel.");
            channels0 = channels0.subcommand(mcmd);
        }
        let mut chromeosdevices0 = SubCommand::with_name("chromeosdevices")
            .setting(AppSettings::ColoredHelp)
            .about("methods: action, get, list, move_devices_to_ou, patch and update");
        {
            let mcmd = SubCommand::with_name("action").about("Takes an action that affects a Chrome OS Device. This includes deprovisioning, disabling, and re-enabling devices. *Warning:* * Deprovisioning a device will stop device policy syncing and remove device-level printers. After a device is deprovisioned, it must be wiped before it can be re-enrolled. * Lost or stolen devices should use the disable action. * Re-enabling a disabled device will consume a device license. If you do not have sufficient licenses available when completing the re-enable action, you will receive an error. For more information about deprovisioning and disabling devices, visit the [help center](https://support.google.com/chrome/a/answer/3523633).");
            chromeosdevices0 = chromeosdevices0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Retrieves a Chrome OS device\'s properties.");
            chromeosdevices0 = chromeosdevices0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a paginated list of Chrome OS devices within an account.");
            chromeosdevices0 = chromeosdevices0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("move_devices_to_ou").about("Move or insert multiple Chrome OS devices to an organizational unit. You can move up to 50 devices at once.");
            chromeosdevices0 = chromeosdevices0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a device\'s updatable properties, such as `annotatedUser`, `annotatedLocation`, `notes`, `orgUnitPath`, or `annotatedAssetId`. This method supports [patch semantics](/admin-sdk/directory/v1/guides/performance#patch).");
            chromeosdevices0 = chromeosdevices0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a device\'s updatable properties, such as `annotatedUser`, `annotatedLocation`, `notes`, `orgUnitPath`, or `annotatedAssetId`.");
            chromeosdevices0 = chromeosdevices0.subcommand(mcmd);
        }
        let mut customer0 = SubCommand::with_name("customer")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: devices");
        let mut customers0 = SubCommand::with_name("customers")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, patch and update");
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves a customer.");
            customers0 = customers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Patch Customers via Apiary Patch Orchestration");
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
                SubCommand::with_name("delete").about("Deletes a domain Alias of the customer.");
            domain_aliases0 = domain_aliases0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Retrieves a domain alias of the customer.");
            domain_aliases0 = domain_aliases0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("insert").about("Inserts a domain alias of the customer.");
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
            let mcmd = SubCommand::with_name("delete").about("Deletes a group.");
            groups0 = groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves a group\'s properties.");
            groups0 = groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a group.");
            groups0 = groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieve all groups of a domain or of a user given a userKey (paginated)");
            groups0 = groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a group\'s properties. This method supports [patch semantics](/admin-sdk/directory/v1/guides/performance#patch).");
            groups0 = groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a group\'s properties.");
            groups0 = groups0.subcommand(mcmd);
        }
        let mut members0 = SubCommand::with_name("members")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, has_member, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Removes a member from a group.");
            members0 = members0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Retrieves a group member\'s properties.");
            members0 = members0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("has_member").about("Checks whether the given user is a member of the group. Membership can be direct or nested.");
            members0 = members0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Adds a user to the specified group.");
            members0 = members0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a paginated list of all members in a group.");
            members0 = members0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the membership properties of a user in the specified group. This method supports [patch semantics](/admin-sdk/directory/v1/guides/performance#patch).");
            members0 = members0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update")
                .about("Updates the membership of a user in the specified group.");
            members0 = members0.subcommand(mcmd);
        }
        let mut mobiledevices0 = SubCommand::with_name("mobiledevices")
            .setting(AppSettings::ColoredHelp)
            .about("methods: action, delete, get and list");
        {
            let mcmd = SubCommand::with_name("action").about("Takes an action that affects a mobile device. For example, remotely wiping a device.");
            mobiledevices0 = mobiledevices0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Removes a mobile device.");
            mobiledevices0 = mobiledevices0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Retrieves a mobile device\'s properties.");
            mobiledevices0 = mobiledevices0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a paginated list of all mobile devices for an account.");
            mobiledevices0 = mobiledevices0.subcommand(mcmd);
        }
        let mut orgunits0 = SubCommand::with_name("orgunits")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Removes an organizational unit.");
            orgunits0 = orgunits0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves an organizational unit.");
            orgunits0 = orgunits0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Adds an organizational unit.");
            orgunits0 = orgunits0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a list of all organizational units for an account.");
            orgunits0 = orgunits0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an organizational unit. This method supports [patch semantics](/admin-sdk/directory/v1/guides/performance#patch)");
            orgunits0 = orgunits0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates an organizational unit.");
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
            let mcmd =
                SubCommand::with_name("patch").about("Patch role via Apiary Patch Orchestration");
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
            let mcmd =
                SubCommand::with_name("patch").about("Patch Schema via Apiary Patch Orchestration");
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
        let mut two_step_verification0 = SubCommand::with_name("two_step_verification")
            .setting(AppSettings::ColoredHelp)
            .about("methods: turn_off");
        {
            let mcmd =
                SubCommand::with_name("turn_off").about("Turn off 2-Step Verification for user.");
            two_step_verification0 = two_step_verification0.subcommand(mcmd);
        }
        let mut users0 = SubCommand::with_name("users")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: delete, get, insert, list, make_admin, patch, sign_out, undelete, update and watch");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a user.");
            users0 = users0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves a user.");
            users0 = users0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a user.");
            users0 = users0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Retrieves a paginated list of either deleted users or all users in a domain.",
            );
            users0 = users0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("make_admin").about("Makes a user a super administrator.");
            users0 = users0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a user using patch semantics. The update method should be used instead, since it also supports patch semantics and has better performance. This method is unable to clear fields that contain repeated objects (`addresses`, `phones`, etc). Use the update method instead.");
            users0 = users0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("sign_out").about("Sign a user out of all web and device sessions and reset their sign-in cookies. User will have to sign in by authenticating again.");
            users0 = users0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("undelete").about("Undeletes a deleted user.");
            users0 = users0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a user. This method supports patch semantics, meaning you only need to include the fields you wish to update. Fields that are not present in the request will be preserved, and fields set to `null` will be cleared.");
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
        let mut devices1 = SubCommand::with_name("devices")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: chromeos");
        let mut chrome1 = SubCommand::with_name("chrome")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: printers");
        let mut aliases1 = SubCommand::with_name("aliases")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, insert and list");
        {
            let mcmd = SubCommand::with_name("delete").about("Removes an alias.");
            aliases1 = aliases1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Adds an alias for the group.");
            aliases1 = aliases1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all aliases for a group.");
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
                .about("Patches a building via Apiary Patch Orchestration.");
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
            let mcmd = SubCommand::with_name("patch")
                .about("Patches a calendar resource via Apiary Patch Orchestration.");
            calendars1 = calendars1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a calendar resource. This method supports patch semantics, meaning you only need to include the fields you wish to update. Fields that are not present in the request will be preserved.");
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
                .about("Patches a feature via Apiary Patch Orchestration.");
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
            let mcmd = SubCommand::with_name("delete").about("Removes an alias.");
            aliases1 = aliases1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Adds an alias.");
            aliases1 = aliases1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all aliases for a user.");
            aliases1 = aliases1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("watch").about("Watch for changes in users list.");
            aliases1 = aliases1.subcommand(mcmd);
        }
        let mut photos1 = SubCommand::with_name("photos")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, patch and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Removes the user\'s photo.");
            photos1 = photos1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves the user\'s photo.");
            photos1 = photos1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Adds a photo for the user. This method supports [patch semantics](/admin-sdk/directory/v1/guides/performance#patch).");
            photos1 = photos1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Adds a photo for the user.");
            photos1 = photos1.subcommand(mcmd);
        }
        let mut chromeos2 = SubCommand::with_name("chromeos")
            .setting(AppSettings::ColoredHelp)
            .about("methods: issue_command");
        {
            let mcmd = SubCommand::with_name("issue_command")
                .about("Issues a command for the device to execute.");
            chromeos2 = chromeos2.subcommand(mcmd);
        }
        let mut printers2 = SubCommand::with_name("printers")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: batch_create_printers, batch_delete_printers, create, delete, get, list, list_printer_models and patch");
        {
            let mcmd = SubCommand::with_name("batch_create_printers")
                .about("Creates printers under given Organization Unit.");
            printers2 = printers2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("batch_delete_printers").about("Deletes printers in batch.");
            printers2 = printers2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a printer under given Organization Unit.");
            printers2 = printers2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a `Printer`.");
            printers2 = printers2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Returns a `Printer` resource (printer\'s config).");
            printers2 = printers2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List printers configs.");
            printers2 = printers2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list_printer_models")
                .about("Lists the supported printer models.");
            printers2 = printers2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a `Printer` resource.");
            printers2 = printers2.subcommand(mcmd);
        }
        let mut commands3 = SubCommand::with_name("commands")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets command data a specific command issued to the device.");
            commands3 = commands3.subcommand(mcmd);
        }
        chromeos2 = chromeos2.subcommand(commands3);
        chrome1 = chrome1.subcommand(printers2);
        devices1 = devices1.subcommand(chromeos2);
        users0 = users0.subcommand(photos1);
        users0 = users0.subcommand(aliases1);
        resources0 = resources0.subcommand(features1);
        resources0 = resources0.subcommand(calendars1);
        resources0 = resources0.subcommand(buildings1);
        groups0 = groups0.subcommand(aliases1);
        customers0 = customers0.subcommand(chrome1);
        customer0 = customer0.subcommand(devices1);
        app = app.subcommand(verification_codes0);
        app = app.subcommand(users0);
        app = app.subcommand(two_step_verification0);
        app = app.subcommand(tokens0);
        app = app.subcommand(schemas0);
        app = app.subcommand(roles0);
        app = app.subcommand(role_assignments0);
        app = app.subcommand(resources0);
        app = app.subcommand(privileges0);
        app = app.subcommand(orgunits0);
        app = app.subcommand(mobiledevices0);
        app = app.subcommand(members0);
        app = app.subcommand(groups0);
        app = app.subcommand(domains0);
        app = app.subcommand(domain_aliases0);
        app = app.subcommand(customers0);
        app = app.subcommand(customer0);
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
