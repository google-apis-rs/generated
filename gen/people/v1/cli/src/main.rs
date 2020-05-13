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
        let mut app = App::new("people1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200512")
            .about("Provides access to information about profiles and contacts.")
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
        let mut contact_groups0 = SubCommand::with_name("contact_groups")
            .setting(AppSettings::ColoredHelp)
            .about("methods: batch_get, create, delete, get, list and update");
        {
            let mcmd = SubCommand::with_name("batch_get").about("Get a list of contact groups owned by the authenticated user by specifying\na list of contact group resource names.");
            contact_groups0 = contact_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create")
                .about("Create a new contact group owned by the authenticated user.");
            contact_groups0 = contact_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Delete an existing contact group owned by the authenticated user by\nspecifying a contact group resource name.");
            contact_groups0 = contact_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Get a specific contact group owned by the authenticated user by specifying\na contact group resource name.");
            contact_groups0 = contact_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List all contact groups owned by the authenticated user. Members of the\ncontact groups are not populated.");
            contact_groups0 = contact_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about(
                "Update the name of an existing contact group owned by the authenticated\nuser.",
            );
            contact_groups0 = contact_groups0.subcommand(mcmd);
        }
        let mut people0 = SubCommand::with_name("people")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create_contact, delete_contact, delete_contact_photo, get, get_batch_get, update_contact and update_contact_photo");
        {
            let mcmd = SubCommand::with_name("create_contact")
                .about("Create a new contact and return the person resource for that contact.");
            people0 = people0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete_contact")
                .about("Delete a contact person. Any non-contact data will not be deleted.");
            people0 = people0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete_contact_photo").about("Delete a contact\'s photo.");
            people0 = people0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Provides information about a person by specifying a resource name. Use\n`people/me` to indicate the authenticated user.\n\nThe request throws a 400 error if \'personFields\' is not specified.");
            people0 = people0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_batch_get").about("Provides information about a list of specific people by specifying a list\nof requested resource names. Use `people/me` to indicate the authenticated\nuser.\n\nThe request throws a 400 error if \'personFields\' is not specified.");
            people0 = people0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_contact").about("Update contact data for an existing contact person. Any non-contact data\nwill not be modified.\n\nThe request throws a 400 error if `updatePersonFields` is not specified.\n\nThe request throws a 400 error if `person.metadata.sources` is not\nspecified for the contact to be updated.\n\nThe request throws a 400 error with an error with reason\n`\"failedPrecondition\"` if `person.metadata.sources.etag` is different than\nthe contact\'s etag, which indicates the contact has changed since its data\nwas read. Clients should get the latest person and re-apply their updates\nto the latest person.");
            people0 = people0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("update_contact_photo").about("Update a contact\'s photo.");
            people0 = people0.subcommand(mcmd);
        }
        let mut members1 = SubCommand::with_name("members")
            .setting(AppSettings::ColoredHelp)
            .about("methods: modify");
        {
            let mcmd = SubCommand::with_name("modify").about("Modify the members of a contact group owned by the authenticated user.\n\nThe only system contact groups that can have members added are\n`contactGroups/myContacts` and `contactGroups/starred`. Other system\ncontact groups are deprecated and can only have contacts removed.");
            members1 = members1.subcommand(mcmd);
        }
        let mut connections1 = SubCommand::with_name("connections")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Provides a list of the authenticated user\'s contacts merged with any\nconnected profiles.\n\nThe request throws a 400 error if \'personFields\' is not specified.");
            connections1 = connections1.subcommand(mcmd);
        }
        people0 = people0.subcommand(connections1);
        contact_groups0 = contact_groups0.subcommand(members1);
        app = app.subcommand(people0);
        app = app.subcommand(contact_groups0);

        Self { app }
    }
}
use google_people1 as api;

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
