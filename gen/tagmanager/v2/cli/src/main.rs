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
        let mut app = App::new("tagmanager2")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20210310")
            .about("This API allows clients to access and modify container and tag configuration.")
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
        let mut accounts0 = SubCommand::with_name("accounts")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, list and update");
        {
            let mcmd = SubCommand::with_name("get").about("Gets a GTM Account.");
            accounts0 = accounts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all GTM Accounts that a user has access to.");
            accounts0 = accounts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a GTM Account.");
            accounts0 = accounts0.subcommand(mcmd);
        }
        let mut containers1 = SubCommand::with_name("containers")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and update");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a Container.");
            containers1 = containers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a Container.");
            containers1 = containers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a Container.");
            containers1 = containers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all Containers that belongs to a GTM Account.");
            containers1 = containers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a Container.");
            containers1 = containers1.subcommand(mcmd);
        }
        let mut user_permissions1 = SubCommand::with_name("user_permissions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and update");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a user\'s Account & Container access.");
            user_permissions1 = user_permissions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about(
                "Removes a user from the account, revoking access to it and all of its containers.",
            );
            user_permissions1 = user_permissions1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets a user\'s Account & Container access.");
            user_permissions1 = user_permissions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List all users that have access to the account along with Account and Container user access granted to each of them.");
            user_permissions1 = user_permissions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update")
                .about("Updates a user\'s Account & Container access.");
            user_permissions1 = user_permissions1.subcommand(mcmd);
        }
        let mut environments2 = SubCommand::with_name("environments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, reauthorize and update");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a GTM Environment.");
            environments2 = environments2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a GTM Environment.");
            environments2 = environments2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a GTM Environment.");
            environments2 = environments2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all GTM Environments of a GTM Container.");
            environments2 = environments2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("reauthorize")
                .about("Re-generates the authorization code for a GTM Environment.");
            environments2 = environments2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a GTM Environment.");
            environments2 = environments2.subcommand(mcmd);
        }
        let mut version_headers2 = SubCommand::with_name("version_headers")
            .setting(AppSettings::ColoredHelp)
            .about("methods: latest and list");
        {
            let mcmd =
                SubCommand::with_name("latest").about("Gets the latest container version header");
            version_headers2 = version_headers2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all Container Versions of a GTM Container.");
            version_headers2 = version_headers2.subcommand(mcmd);
        }
        let mut versions2 = SubCommand::with_name("versions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, live, publish, set_latest, undelete and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a Container Version.");
            versions2 = versions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a Container Version.");
            versions2 = versions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("live")
                .about("Gets the live (i.e. published) container version");
            versions2 = versions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("publish").about("Publishes a Container Version.");
            versions2 = versions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_latest").about("Sets the latest version used for synchronization of workspaces when detecting conflicts and errors.");
            versions2 = versions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("undelete").about("Undeletes a Container Version.");
            versions2 = versions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a Container Version.");
            versions2 = versions2.subcommand(mcmd);
        }
        let mut workspaces2 = SubCommand::with_name("workspaces")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, create_version, delete, get, get_status, list, quick_preview, resolve_conflict, sync and update");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a Workspace.");
            workspaces2 = workspaces2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create_version").about("Creates a Container Version from the entities present in the workspace, deletes the workspace, and sets the base container version to the newly created version.");
            workspaces2 = workspaces2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a Workspace.");
            workspaces2 = workspaces2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a Workspace.");
            workspaces2 = workspaces2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_status")
                .about("Finds conflicting and modified entities in the workspace.");
            workspaces2 = workspaces2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all Workspaces that belong to a GTM Container.");
            workspaces2 = workspaces2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("quick_preview").about("Quick previews a workspace by creating a fake container version from all entities in the provided workspace.");
            workspaces2 = workspaces2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("resolve_conflict").about("Resolves a merge conflict for a workspace entity by updating it to the resolved entity passed in the request.");
            workspaces2 = workspaces2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("sync").about("Syncs a workspace to the latest container version by updating all unmodified workspace entities and displaying conflicts for modified entities.");
            workspaces2 = workspaces2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a Workspace.");
            workspaces2 = workspaces2.subcommand(mcmd);
        }
        let mut built_in_variables3 = SubCommand::with_name("built_in_variables")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, list and revert");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates one or more GTM Built-In Variables.");
            built_in_variables3 = built_in_variables3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes one or more GTM Built-In Variables.");
            built_in_variables3 = built_in_variables3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all the enabled Built-In Variables of a GTM Container.");
            built_in_variables3 = built_in_variables3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("revert")
                .about("Reverts changes to a GTM Built-In Variables in a GTM Workspace.");
            built_in_variables3 = built_in_variables3.subcommand(mcmd);
        }
        let mut folders3 = SubCommand::with_name("folders")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, entities, get, list, move_entities_to_folder, revert and update");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a GTM Folder.");
            folders3 = folders3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a GTM Folder.");
            folders3 = folders3.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("entities").about("List all entities in a GTM Folder.");
            folders3 = folders3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a GTM Folder.");
            folders3 = folders3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all GTM Folders of a Container.");
            folders3 = folders3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("move_entities_to_folder")
                .about("Moves entities to a GTM Folder.");
            folders3 = folders3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("revert")
                .about("Reverts changes to a GTM Folder in a GTM Workspace.");
            folders3 = folders3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a GTM Folder.");
            folders3 = folders3.subcommand(mcmd);
        }
        let mut tags3 = SubCommand::with_name("tags")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, revert and update");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a GTM Tag.");
            tags3 = tags3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a GTM Tag.");
            tags3 = tags3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a GTM Tag.");
            tags3 = tags3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all GTM Tags of a Container.");
            tags3 = tags3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("revert")
                .about("Reverts changes to a GTM Tag in a GTM Workspace.");
            tags3 = tags3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a GTM Tag.");
            tags3 = tags3.subcommand(mcmd);
        }
        let mut templates3 = SubCommand::with_name("templates")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, revert and update");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a GTM Custom Template.");
            templates3 = templates3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a GTM Template.");
            templates3 = templates3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a GTM Template.");
            templates3 = templates3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all GTM Templates of a GTM container workspace.");
            templates3 = templates3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("revert")
                .about("Reverts changes to a GTM Template in a GTM Workspace.");
            templates3 = templates3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a GTM Template.");
            templates3 = templates3.subcommand(mcmd);
        }
        let mut triggers3 = SubCommand::with_name("triggers")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, revert and update");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a GTM Trigger.");
            triggers3 = triggers3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a GTM Trigger.");
            triggers3 = triggers3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a GTM Trigger.");
            triggers3 = triggers3.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists all GTM Triggers of a Container.");
            triggers3 = triggers3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("revert")
                .about("Reverts changes to a GTM Trigger in a GTM Workspace.");
            triggers3 = triggers3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a GTM Trigger.");
            triggers3 = triggers3.subcommand(mcmd);
        }
        let mut variables3 = SubCommand::with_name("variables")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, revert and update");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a GTM Variable.");
            variables3 = variables3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a GTM Variable.");
            variables3 = variables3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a GTM Variable.");
            variables3 = variables3.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists all GTM Variables of a Container.");
            variables3 = variables3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("revert")
                .about("Reverts changes to a GTM Variable in a GTM Workspace.");
            variables3 = variables3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a GTM Variable.");
            variables3 = variables3.subcommand(mcmd);
        }
        let mut zones3 = SubCommand::with_name("zones")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, revert and update");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a GTM Zone.");
            zones3 = zones3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a GTM Zone.");
            zones3 = zones3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a GTM Zone.");
            zones3 = zones3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all GTM Zones of a GTM container workspace.");
            zones3 = zones3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("revert")
                .about("Reverts changes to a GTM Zone in a GTM Workspace.");
            zones3 = zones3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a GTM Zone.");
            zones3 = zones3.subcommand(mcmd);
        }
        workspaces2 = workspaces2.subcommand(zones3);
        workspaces2 = workspaces2.subcommand(variables3);
        workspaces2 = workspaces2.subcommand(triggers3);
        workspaces2 = workspaces2.subcommand(templates3);
        workspaces2 = workspaces2.subcommand(tags3);
        workspaces2 = workspaces2.subcommand(folders3);
        workspaces2 = workspaces2.subcommand(built_in_variables3);
        containers1 = containers1.subcommand(workspaces2);
        containers1 = containers1.subcommand(versions2);
        containers1 = containers1.subcommand(version_headers2);
        containers1 = containers1.subcommand(environments2);
        accounts0 = accounts0.subcommand(user_permissions1);
        accounts0 = accounts0.subcommand(containers1);
        app = app.subcommand(accounts0);

        Self { app }
    }
}
use google_tagmanager2 as api;

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
