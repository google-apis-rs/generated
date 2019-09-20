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
        let mut app = App::new("androiddeviceprovisioning1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20190912")
            .about("Automates Android zero-touch enrollment for device resellers, customers, and EMMs.")
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
        let mut customers0 = SubCommand::with_name("customers")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists the user\'s customer accounts.");
            customers0 = customers0.subcommand(mcmd);
        }
        let mut operations0 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation.  Clients can use this\nmethod to poll the operation result at intervals as recommended by the API\nservice.");
            operations0 = operations0.subcommand(mcmd);
        }
        let mut partners0 = SubCommand::with_name("partners")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: customers, devices and vendors");
        let mut configurations1 = SubCommand::with_name("configurations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new configuration. Once created, a customer can apply the\nconfiguration to devices.");
            configurations1 = configurations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an unused configuration. The API call fails if the customer has\ndevices with the configuration applied.");
            configurations1 = configurations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the details of a configuration.");
            configurations1 = configurations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists a customer\'s configurations.");
            configurations1 = configurations1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("patch").about("Updates a configuration\'s field values.");
            configurations1 = configurations1.subcommand(mcmd);
        }
        let mut devices1 = SubCommand::with_name("devices")
            .setting(AppSettings::ColoredHelp)
            .about("methods: apply_configuration, get, list, remove_configuration and unclaim");
        {
            let mcmd = SubCommand::with_name("apply_configuration").about("Applies a Configuration to the device to register the device for zero-touch\nenrollment. After applying a configuration to a device, the device\nautomatically provisions itself on first boot, or next factory reset.");
            devices1 = devices1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the details of a device.");
            devices1 = devices1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists a customer\'s devices.");
            devices1 = devices1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("remove_configuration")
                .about("Removes a configuration from device.");
            devices1 = devices1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("unclaim").about("Unclaims a device from a customer and removes it from zero-touch\nenrollment.\n\nAfter removing a device, a customer must contact their reseller to register\nthe device into zero-touch enrollment again.");
            devices1 = devices1.subcommand(mcmd);
        }
        let mut dpcs1 = SubCommand::with_name("dpcs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about(
                "Lists the DPCs (device policy controllers) that support zero-touch\nenrollment.",
            );
            dpcs1 = dpcs1.subcommand(mcmd);
        }
        let mut customers1 = SubCommand::with_name("customers")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create and list");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a customer for zero-touch enrollment. After the method returns\nsuccessfully, admin and owner roles can manage devices and EMM configs\nby calling API methods or using their zero-touch enrollment portal.\nThe customer receives an email that welcomes them to zero-touch enrollment\nand explains how to sign into the portal.");
            customers1 = customers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the customers that are enrolled to the reseller identified by the\n`partnerId` argument. This list includes customers that the reseller\ncreated and customers that enrolled themselves using the portal.");
            customers1 = customers1.subcommand(mcmd);
        }
        let mut devices1 = SubCommand::with_name("devices")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: claim, claim_async, find_by_identifier, find_by_owner, get, metadata, unclaim, unclaim_async and update_metadata_async");
        {
            let mcmd = SubCommand::with_name("claim").about("Claims a device for a customer and adds it to zero-touch enrollment. If the\ndevice is already claimed by another customer, the call returns an error.");
            devices1 = devices1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("claim_async").about("Claims a batch of devices for a customer asynchronously. Adds the devices\nto zero-touch enrollment. To learn more, read [Long‑running batch\noperations](/zero-touch/guides/how-it-works#operations).");
            devices1 = devices1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("find_by_identifier")
                .about("Finds devices by hardware identifiers, such as IMEI.");
            devices1 = devices1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("find_by_owner").about("Finds devices claimed for customers. The results only contain devices\nregistered to the reseller that\'s identified by the `partnerId` argument.\nThe customer\'s devices purchased from other resellers don\'t appear in the\nresults.");
            devices1 = devices1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a device.");
            devices1 = devices1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("metadata")
                .about("Updates reseller metadata associated with the device.");
            devices1 = devices1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("unclaim").about(
                "Unclaims a device from a customer and removes it from zero-touch\nenrollment.",
            );
            devices1 = devices1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("unclaim_async").about("Unclaims a batch of devices for a customer asynchronously. Removes the\ndevices from zero-touch enrollment. To learn more, read [Long‑running batch\noperations](/zero-touch/guides/how-it-works#operations).");
            devices1 = devices1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_metadata_async").about("Updates the reseller metadata attached to a batch of devices. This method\nupdates devices asynchronously and returns an `Operation` that can be used\nto track progress. Read [Long‑running batch\noperations](/zero-touch/guides/how-it-works#operations).");
            devices1 = devices1.subcommand(mcmd);
        }
        let mut vendors1 = SubCommand::with_name("vendors")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists the vendors of the partner.");
            vendors1 = vendors1.subcommand(mcmd);
        }
        let mut customers2 = SubCommand::with_name("customers")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists the customers of the vendor.");
            customers2 = customers2.subcommand(mcmd);
        }
        vendors1 = vendors1.subcommand(customers2);
        partners0 = partners0.subcommand(vendors1);
        partners0 = partners0.subcommand(devices1);
        partners0 = partners0.subcommand(customers1);
        customers0 = customers0.subcommand(dpcs1);
        customers0 = customers0.subcommand(devices1);
        customers0 = customers0.subcommand(configurations1);
        app = app.subcommand(partners0);
        app = app.subcommand(operations0);
        app = app.subcommand(customers0);

        Self { app }
    }
}
use google_androiddeviceprovisioning1 as api;

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
