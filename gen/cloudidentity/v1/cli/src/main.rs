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
        let mut app = App::new("cloudidentity1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20210310")
            .about("API for provisioning and managing identity resources.")
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
        let mut devices0 = SubCommand::with_name("devices")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel_wipe, create, delete, get, list and wipe");
        {
            let mcmd = SubCommand::with_name("cancel_wipe").about("Cancels an unfinished device wipe. This operation can be used to cancel device wipe in the gap between the wipe operation returning success and the device being wiped. This operation is possible when the device is in a \"pending wipe\" state. The device enters the \"pending wipe\" state when a wipe device command is issued, but has not yet been sent to the device. The cancel wipe will fail if the wipe command has already been issued to the device.");
            devices0 = devices0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a device. Only company-owned device may be created. **Note**: This method is available only to customers who have one of the following SKUs: Enterprise Standard, Enterprise Plus, Enterprise for Education, and Cloud Identity Premium");
            devices0 = devices0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified device.");
            devices0 = devices0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves the specified device.");
            devices0 = devices0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists/Searches devices.");
            devices0 = devices0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("wipe").about("Wipes all data on the specified device.");
            devices0 = devices0.subcommand(mcmd);
        }
        let mut groups0 = SubCommand::with_name("groups")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, lookup, patch and search");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a Group.");
            groups0 = groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a `Group`.");
            groups0 = groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves a `Group`.");
            groups0 = groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists the `Group`s under a customer or namespace.");
            groups0 = groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("lookup").about("Looks up the [resource name](https://cloud.google.com/apis/design/resource_names) of a `Group` by its `EntityKey`.");
            groups0 = groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a `Group`.");
            groups0 = groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("search")
                .about("Searches for `Group`s matching a specified query.");
            groups0 = groups0.subcommand(mcmd);
        }
        let mut device_users1 = SubCommand::with_name("device_users")
            .setting(AppSettings::ColoredHelp)
            .about("methods: approve, block, cancel_wipe, delete, get, list, lookup and wipe");
        {
            let mcmd =
                SubCommand::with_name("approve").about("Approves device to access user data.");
            device_users1 = device_users1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("block").about("Blocks device from accessing user data");
            device_users1 = device_users1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("cancel_wipe").about("Cancels an unfinished user account wipe. This operation can be used to cancel device wipe in the gap between the wipe operation returning success and the device being wiped.");
            device_users1 = device_users1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified DeviceUser. This also revokes the user\'s access to device data.");
            device_users1 = device_users1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves the specified DeviceUser");
            device_users1 = device_users1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists/Searches DeviceUsers.");
            device_users1 = device_users1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("lookup").about("Looks up resource names of the DeviceUsers associated with the caller\'s credentials, as well as the properties provided in the request. This method must be called with end-user credentials with the scope: https://www.googleapis.com/auth/cloud-identity.devices.lookup If multiple properties are provided, only DeviceUsers having all of these properties are considered as matches - i.e. the query behaves like an AND. Different platforms require different amounts of information from the caller to ensure that the DeviceUser is uniquely identified. - iOS: No properties need to be passed, the caller\'s credentials are sufficient to identify the corresponding DeviceUser. - Android: Specifying the \'android_id\' field is required. - Desktop: Specifying the \'raw_resource_id\' field is required.");
            device_users1 = device_users1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("wipe").about("Wipes the user\'s account on a device. Other data on the device that is not associated with the user\'s work account is not affected. For example, if a Gmail app is installed on a device that is used for personal and work purposes, and the user is logged in to the Gmail app with their personal account as well as their work account, wiping the \"deviceUser\" by their work administrator will not affect their personal account within Gmail or other apps such as Photos.");
            device_users1 = device_users1.subcommand(mcmd);
        }
        let mut memberships1 = SubCommand::with_name("memberships")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: check_transitive_membership, create, delete, get, get_membership_graph, list, lookup, modify_membership_roles, search_transitive_groups and search_transitive_memberships");
        {
            let mcmd = SubCommand::with_name("check_transitive_membership").about("Check a potential member for membership in a group. **Note:** This feature is only available to Google Workspace Enterprise Standard, Enterprise Plus, and Enterprise for Education; and Cloud Identity Premium accounts. If the account of the member is not one of these, a 403 (PERMISSION_DENIED) HTTP status code will be returned. A member has membership to a group as long as there is a single viewable transitive membership between the group and the member. The actor must have view permissions to at least one transitive membership between the member and group.");
            memberships1 = memberships1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a `Membership`.");
            memberships1 = memberships1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a `Membership`.");
            memberships1 = memberships1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves a `Membership`.");
            memberships1 = memberships1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_membership_graph").about("Get a membership graph of just a member or both a member and a group. **Note:** This feature is only available to Google Workspace Enterprise Standard, Enterprise Plus, and Enterprise for Education; and Cloud Identity Premium accounts. If the account of the member is not one of these, a 403 (PERMISSION_DENIED) HTTP status code will be returned. Given a member, the response will contain all membership paths from the member. Given both a group and a member, the response will contain all membership paths between the group and the member.");
            memberships1 = memberships1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists the `Membership`s within a `Group`.");
            memberships1 = memberships1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("lookup").about("Looks up the [resource name](https://cloud.google.com/apis/design/resource_names) of a `Membership` by its `EntityKey`.");
            memberships1 = memberships1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("modify_membership_roles")
                .about("Modifies the `MembershipRole`s of a `Membership`.");
            memberships1 = memberships1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("search_transitive_groups").about("Search transitive groups of a member. **Note:** This feature is only available to Google Workspace Enterprise Standard, Enterprise Plus, and Enterprise for Education; and Cloud Identity Premium accounts. If the account of the member is not one of these, a 403 (PERMISSION_DENIED) HTTP status code will be returned. A transitive group is any group that has a direct or indirect membership to the member. Actor must have view permissions all transitive groups.");
            memberships1 = memberships1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("search_transitive_memberships").about("Search transitive memberships of a group. **Note:** This feature is only available to Google Workspace Enterprise Standard, Enterprise Plus, and Enterprise for Education; and Cloud Identity Premium accounts. If the account of the group is not one of these, a 403 (PERMISSION_DENIED) HTTP status code will be returned. A transitive membership is any direct or indirect membership of a group. Actor must have view permissions to all transitive memberships.");
            memberships1 = memberships1.subcommand(mcmd);
        }
        let mut client_states2 = SubCommand::with_name("client_states")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, list and patch");
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets the client state for the device user");
            client_states2 = client_states2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists the client states for the given search query.");
            client_states2 = client_states2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the client state for the device user **Note**: This method is available only to customers who have one of the following SKUs: Enterprise Standard, Enterprise Plus, Enterprise for Education, and Cloud Identity Premium");
            client_states2 = client_states2.subcommand(mcmd);
        }
        device_users1 = device_users1.subcommand(client_states2);
        groups0 = groups0.subcommand(memberships1);
        devices0 = devices0.subcommand(device_users1);
        app = app.subcommand(groups0);
        app = app.subcommand(devices0);

        Self { app }
    }
}
use google_cloudidentity1 as api;

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
