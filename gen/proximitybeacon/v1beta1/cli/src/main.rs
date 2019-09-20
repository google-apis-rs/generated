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
        let mut app = App::new("proximitybeacon1_beta1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20190323")
            .about("Registers, manages, indexes, and searches beacons.")
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
        let mut beaconinfo0 = SubCommand::with_name("beaconinfo")
            .setting(AppSettings::ColoredHelp)
            .about("methods: getforobserved");
        {
            let mcmd = SubCommand::with_name("getforobserved").about("Given one or more beacon observations, returns any beacon information\nand attachments accessible to your application. Authorize by using the\n[API key](https://developers.google.com/beacons/proximity/get-started#request_a_browser_api_key)\nfor the application.");
            beaconinfo0 = beaconinfo0.subcommand(mcmd);
        }
        let mut beacons0 = SubCommand::with_name("beacons")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: activate, deactivate, decommission, delete, get, list, register and update");
        {
            let mcmd = SubCommand::with_name("activate").about("Activates a beacon. A beacon that is active will return information\nand attachment data when queried via `beaconinfo.getforobserved`.\nCalling this method on an already active beacon will do nothing (but\nwill return a successful response code).\n\nAuthenticate using an [OAuth access token](https://developers.google.com/identity/protocols/OAuth2)\nfrom a signed-in user with **Is owner** or **Can edit** permissions in the\nGoogle Developers Console project.");
            beacons0 = beacons0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("deactivate").about("Deactivates a beacon. Once deactivated, the API will not return\ninformation nor attachment data for the beacon when queried via\n`beaconinfo.getforobserved`. Calling this method on an already inactive\nbeacon will do nothing (but will return a successful response code).\n\nAuthenticate using an [OAuth access token](https://developers.google.com/identity/protocols/OAuth2)\nfrom a signed-in user with **Is owner** or **Can edit** permissions in the\nGoogle Developers Console project.");
            beacons0 = beacons0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("decommission").about("Decommissions the specified beacon in the service. This beacon will no\nlonger be returned from `beaconinfo.getforobserved`. This operation is\npermanent -- you will not be able to re-register a beacon with this ID\nagain.\n\nAuthenticate using an [OAuth access token](https://developers.google.com/identity/protocols/OAuth2)\nfrom a signed-in user with **Is owner** or **Can edit** permissions in the\nGoogle Developers Console project.");
            beacons0 = beacons0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified beacon including all diagnostics data for the beacon\nas well as any attachments on the beacon (including those belonging to\nother projects). This operation cannot be undone.\n\nAuthenticate using an [OAuth access token](https://developers.google.com/identity/protocols/OAuth2)\nfrom a signed-in user with **Is owner** or **Can edit** permissions in the\nGoogle Developers Console project.");
            beacons0 = beacons0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns detailed information about the specified beacon.\n\nAuthenticate using an [OAuth access token](https://developers.google.com/identity/protocols/OAuth2)\nfrom a signed-in user with **viewer**, **Is owner** or **Can edit**\npermissions in the Google Developers Console project.\n\nRequests may supply an Eddystone-EID beacon name in the form:\n`beacons/4!beaconId` where the `beaconId` is the base16 ephemeral ID\nbroadcast by the beacon. The returned `Beacon` object will contain the\nbeacon\'s stable Eddystone-UID. Clients not authorized to resolve the\nbeacon\'s ephemeral Eddystone-EID broadcast will receive an error.");
            beacons0 = beacons0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Searches the beacon registry for beacons that match the given search\ncriteria. Only those beacons that the client has permission to list\nwill be returned.\n\nAuthenticate using an [OAuth access token](https://developers.google.com/identity/protocols/OAuth2)\nfrom a signed-in user with **viewer**, **Is owner** or **Can edit**\npermissions in the Google Developers Console project.");
            beacons0 = beacons0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("register").about("Registers a previously unregistered beacon given its `advertisedId`.\nThese IDs are unique within the system. An ID can be registered only once.\n\nAuthenticate using an [OAuth access token](https://developers.google.com/identity/protocols/OAuth2)\nfrom a signed-in user with **Is owner** or **Can edit** permissions in the\nGoogle Developers Console project.");
            beacons0 = beacons0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates the information about the specified beacon. **Any field that you do\nnot populate in the submitted beacon will be permanently erased**, so you\nshould follow the \"read, modify, write\" pattern to avoid inadvertently\ndestroying data.\n\nChanges to the beacon status via this method will be  silently ignored.\nTo update beacon status, use the separate methods on this API for\nactivation, deactivation, and decommissioning.\nAuthenticate using an [OAuth access token](https://developers.google.com/identity/protocols/OAuth2)\nfrom a signed-in user with **Is owner** or **Can edit** permissions in the\nGoogle Developers Console project.");
            beacons0 = beacons0.subcommand(mcmd);
        }
        let mut namespaces0 = SubCommand::with_name("namespaces")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list and update");
        {
            let mcmd = SubCommand::with_name("list").about("Lists all attachment namespaces owned by your Google Developers Console\nproject. Attachment data associated with a beacon must include a\nnamespaced type, and the namespace must be owned by your project.\n\nAuthenticate using an [OAuth access token](https://developers.google.com/identity/protocols/OAuth2)\nfrom a signed-in user with **viewer**, **Is owner** or **Can edit**\npermissions in the Google Developers Console project.");
            namespaces0 = namespaces0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates the information about the specified namespace. Only the namespace\nvisibility can be updated.");
            namespaces0 = namespaces0.subcommand(mcmd);
        }
        let mut v_1beta_10 = SubCommand::with_name("v_1beta_1")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get_eidparams");
        {
            let mcmd = SubCommand::with_name("get_eidparams").about("Gets the Proximity Beacon API\'s current public key and associated\nparameters used to initiate the Diffie-Hellman key exchange required to\nregister a beacon that broadcasts the Eddystone-EID format. This key\nchanges periodically; clients may cache it and re-use the same public key\nto provision and register multiple beacons. However, clients should be\nprepared to refresh this key when they encounter an error registering an\nEddystone-EID beacon.");
            v_1beta_10 = v_1beta_10.subcommand(mcmd);
        }
        let mut attachments1 = SubCommand::with_name("attachments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: batch_delete, create, delete and list");
        {
            let mcmd = SubCommand::with_name("batch_delete").about("Deletes multiple attachments on a given beacon. This operation is\npermanent and cannot be undone.\n\nYou can optionally specify `namespacedType` to choose which attachments\nshould be deleted. If you do not specify `namespacedType`,  all your\nattachments on the given beacon will be deleted. You also may explicitly\nspecify `*/*` to delete all.\n\nAuthenticate using an [OAuth access token](https://developers.google.com/identity/protocols/OAuth2)\nfrom a signed-in user with **Is owner** or **Can edit** permissions in the\nGoogle Developers Console project.");
            attachments1 = attachments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Associates the given data with the specified beacon. Attachment data must\ncontain two parts:\n<ul>\n<li>A namespaced type.</li>\n<li>The actual attachment data itself.</li>\n</ul>\nThe namespaced type consists of two parts, the namespace and the type.\nThe namespace must be one of the values returned by the `namespaces`\nendpoint, while the type can be a string of any characters except for the\nforward slash (`/`) up to 100 characters in length.\n\nAttachment data can be up to 1024 bytes long.\n\nAuthenticate using an [OAuth access token](https://developers.google.com/identity/protocols/OAuth2)\nfrom a signed-in user with **Is owner** or **Can edit** permissions in the\nGoogle Developers Console project.");
            attachments1 = attachments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified attachment for the given beacon. Each attachment has\na unique attachment name (`attachmentName`) which is returned when you\nfetch the attachment data via this API. You specify this with the delete\nrequest to control which attachment is removed. This operation cannot be\nundone.\n\nAuthenticate using an [OAuth access token](https://developers.google.com/identity/protocols/OAuth2)\nfrom a signed-in user with **Is owner** or **Can edit** permissions in the\nGoogle Developers Console project.");
            attachments1 = attachments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Returns the attachments for the specified beacon that match the specified\nnamespaced-type pattern.\n\nTo control which namespaced types are returned, you add the\n`namespacedType` query parameter to the request. You must either use\n`*/*`, to return all attachments, or the namespace must be one of\nthe ones returned from the  `namespaces` endpoint.\n\nAuthenticate using an [OAuth access token](https://developers.google.com/identity/protocols/OAuth2)\nfrom a signed-in user with **viewer**, **Is owner** or **Can edit**\npermissions in the Google Developers Console project.");
            attachments1 = attachments1.subcommand(mcmd);
        }
        let mut diagnostics1 = SubCommand::with_name("diagnostics")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("List the diagnostics for a single beacon. You can also list diagnostics for\nall the beacons owned by your Google Developers Console project by using\nthe beacon name `beacons/-`.\n\nAuthenticate using an [OAuth access token](https://developers.google.com/identity/protocols/OAuth2)\nfrom a signed-in user with **viewer**, **Is owner** or **Can edit**\npermissions in the Google Developers Console project.");
            diagnostics1 = diagnostics1.subcommand(mcmd);
        }
        beacons0 = beacons0.subcommand(diagnostics1);
        beacons0 = beacons0.subcommand(attachments1);
        app = app.subcommand(v_1beta_10);
        app = app.subcommand(namespaces0);
        app = app.subcommand(beacons0);
        app = app.subcommand(beaconinfo0);

        Self { app }
    }
}
use google_proximitybeacon1_beta1 as api;

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
