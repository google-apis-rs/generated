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
        let mut app = App::new("cloudiot1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20210309")
            .about("Registers and manages IoT (Internet of Things) devices that connect to the Google Cloud Platform. ")
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
        let mut projects0 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: locations");
        let mut locations1 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: registries");
        let mut registries2 = SubCommand::with_name("registries")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: bind_device_to_gateway, create, delete, get, get_iam_policy, list, patch, set_iam_policy, test_iam_permissions and unbind_device_from_gateway");
        {
            let mcmd = SubCommand::with_name("bind_device_to_gateway")
                .about("Associates the device with the gateway.");
            registries2 = registries2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a device registry that contains devices.");
            registries2 = registries2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes a device registry configuration.");
            registries2 = registries2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a device registry configuration.");
            registries2 = registries2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.");
            registries2 = registries2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists device registries.");
            registries2 = registries2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("patch").about("Updates a device registry configuration.");
            registries2 = registries2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy.");
            registries2 = registries2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a NOT_FOUND error.");
            registries2 = registries2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("unbind_device_from_gateway")
                .about("Deletes the association between the device and the gateway.");
            registries2 = registries2.subcommand(mcmd);
        }
        let mut devices3 = SubCommand::with_name("devices")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, list, modify_cloud_to_device_config, patch and send_command_to_device");
        {
            let mcmd =
                SubCommand::with_name("create").about("Creates a device in a device registry.");
            devices3 = devices3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a device.");
            devices3 = devices3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets details about a device.");
            devices3 = devices3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List devices in a device registry.");
            devices3 = devices3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("modify_cloud_to_device_config").about("Modifies the configuration for the device, which is eventually sent from the Cloud IoT Core servers. Returns the modified configuration version and its metadata.");
            devices3 = devices3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a device.");
            devices3 = devices3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("send_command_to_device").about("Sends a command to the specified device. In order for a device to be able to receive commands, it must: 1) be connected to Cloud IoT Core using the MQTT protocol, and 2) be subscribed to the group of MQTT topics specified by /devices/{device-id}/commands/#. This subscription will receive commands at the top-level topic /devices/{device-id}/commands as well as commands for subfolders, like /devices/{device-id}/commands/subfolder. Note that subscribing to specific subfolders is not supported. If the command could not be delivered to the device, this method will return an error; in particular, if the device is not subscribed, this method will return FAILED_PRECONDITION. Otherwise, this method will return OK. If the subscription is QoS 1, at least once delivery will be guaranteed; for QoS 0, no acknowledgment will be expected from the device.");
            devices3 = devices3.subcommand(mcmd);
        }
        let mut groups3 = SubCommand::with_name("groups")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get_iam_policy, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.");
            groups3 = groups3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy.");
            groups3 = groups3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a NOT_FOUND error.");
            groups3 = groups3.subcommand(mcmd);
        }
        let mut config_versions4 = SubCommand::with_name("config_versions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists the last few versions of the device configuration in descending order (i.e.: newest first).");
            config_versions4 = config_versions4.subcommand(mcmd);
        }
        let mut states4 = SubCommand::with_name("states")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists the last few versions of the device state in descending order (i.e.: newest first).");
            states4 = states4.subcommand(mcmd);
        }
        let mut devices4 = SubCommand::with_name("devices")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("List devices in a device registry.");
            devices4 = devices4.subcommand(mcmd);
        }
        groups3 = groups3.subcommand(devices4);
        devices3 = devices3.subcommand(states4);
        devices3 = devices3.subcommand(config_versions4);
        registries2 = registries2.subcommand(groups3);
        registries2 = registries2.subcommand(devices3);
        locations1 = locations1.subcommand(registries2);
        projects0 = projects0.subcommand(locations1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_cloudiot1 as api;

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
