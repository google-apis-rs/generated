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
        let mut app = App::new("compute1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200606")
            .about("Creates and runs virtual machines on Google Cloud Platform.")
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
        let mut accelerator_types0 = SubCommand::with_name("accelerator_types")
            .setting(AppSettings::ColoredHelp)
            .about("methods: aggregated_list, get and list");
        {
            let mcmd = SubCommand::with_name("aggregated_list")
                .about("Retrieves an aggregated list of accelerator types.");
            accelerator_types0 = accelerator_types0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Returns the specified accelerator type.");
            accelerator_types0 = accelerator_types0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves a list of accelerator types that are available to the specified project.");
            accelerator_types0 = accelerator_types0.subcommand(mcmd);
        }
        let mut addresses0 = SubCommand::with_name("addresses")
            .setting(AppSettings::ColoredHelp)
            .about("methods: aggregated_list, delete, get, insert and list");
        {
            let mcmd = SubCommand::with_name("aggregated_list")
                .about("Retrieves an aggregated list of addresses.");
            addresses0 = addresses0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes the specified address resource.");
            addresses0 = addresses0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Returns the specified address resource.");
            addresses0 = addresses0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates an address resource in the specified project by using the data included in the request.");
            addresses0 = addresses0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a list of addresses contained within the specified region.");
            addresses0 = addresses0.subcommand(mcmd);
        }
        let mut autoscalers0 = SubCommand::with_name("autoscalers")
            .setting(AppSettings::ColoredHelp)
            .about("methods: aggregated_list, delete, get, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("aggregated_list")
                .about("Retrieves an aggregated list of autoscalers.");
            autoscalers0 = autoscalers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified autoscaler.");
            autoscalers0 = autoscalers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified autoscaler resource. Gets a list of available autoscalers by making a list() request.");
            autoscalers0 = autoscalers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates an autoscaler in the specified project using the data included in the request.");
            autoscalers0 = autoscalers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a list of autoscalers contained within the specified zone.");
            autoscalers0 = autoscalers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an autoscaler in the specified project using the data included in the request. This method supports PATCH semantics and uses the JSON merge patch format and processing rules.");
            autoscalers0 = autoscalers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates an autoscaler in the specified project using the data included in the request.");
            autoscalers0 = autoscalers0.subcommand(mcmd);
        }
        let mut backend_buckets0 = SubCommand::with_name("backend_buckets")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: add_signed_url_key, delete, delete_signed_url_key, get, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("add_signed_url_key").about(
                "Adds a key for validating requests with signed URLs for this backend bucket.",
            );
            backend_buckets0 = backend_buckets0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes the specified BackendBucket resource.");
            backend_buckets0 = backend_buckets0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete_signed_url_key").about(
                "Deletes a key for validating requests with signed URLs for this backend bucket.",
            );
            backend_buckets0 = backend_buckets0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified BackendBucket resource. Gets a list of available backend buckets by making a list() request.");
            backend_buckets0 = backend_buckets0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a BackendBucket resource in the specified project using the data included in the request.");
            backend_buckets0 = backend_buckets0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Retrieves the list of BackendBucket resources available to the specified project.",
            );
            backend_buckets0 = backend_buckets0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified BackendBucket resource with the data included in the request. This method supports PATCH semantics and uses the JSON merge patch format and processing rules.");
            backend_buckets0 = backend_buckets0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates the specified BackendBucket resource with the data included in the request.");
            backend_buckets0 = backend_buckets0.subcommand(mcmd);
        }
        let mut backend_services0 = SubCommand::with_name("backend_services")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: add_signed_url_key, aggregated_list, delete, delete_signed_url_key, get, get_health, insert, list, patch, set_security_policy and update");
        {
            let mcmd = SubCommand::with_name("add_signed_url_key").about(
                "Adds a key for validating requests with signed URLs for this backend service.",
            );
            backend_services0 = backend_services0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("aggregated_list").about("Retrieves the list of all BackendService resources, regional and global, available to the specified project.");
            backend_services0 = backend_services0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes the specified BackendService resource.");
            backend_services0 = backend_services0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete_signed_url_key").about(
                "Deletes a key for validating requests with signed URLs for this backend service.",
            );
            backend_services0 = backend_services0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified BackendService resource. Gets a list of available backend services.");
            backend_services0 = backend_services0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_health").about("Gets the most recent health check results for this BackendService.\n\nExample request body:\n\n{ \"group\": \"/zones/us-east1-b/instanceGroups/lb-backend-example\" }");
            backend_services0 = backend_services0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a BackendService resource in the specified project using the data included in the request. There are several restrictions and guidelines to keep in mind when creating a backend service. Read  Understanding backend services for more information.");
            backend_services0 = backend_services0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves the list of BackendService resources available to the specified project.");
            backend_services0 = backend_services0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Patches the specified BackendService resource with the data included in the request. There are several Understanding backend services to keep in mind when updating a backend service. Read  Understanding backend services for more information. This method supports PATCH semantics and uses the JSON merge patch format and processing rules.");
            backend_services0 = backend_services0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_security_policy")
                .about("Sets the security policy for the specified backend service.");
            backend_services0 = backend_services0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates the specified BackendService resource with the data included in the request. There are several Understanding backend services to keep in mind when updating a backend service. Read  Understanding backend services for more information.");
            backend_services0 = backend_services0.subcommand(mcmd);
        }
        let mut disk_types0 = SubCommand::with_name("disk_types")
            .setting(AppSettings::ColoredHelp)
            .about("methods: aggregated_list, get and list");
        {
            let mcmd = SubCommand::with_name("aggregated_list")
                .about("Retrieves an aggregated list of disk types.");
            disk_types0 = disk_types0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified disk type. Gets a list of available disk types by making a list() request.");
            disk_types0 = disk_types0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a list of disk types available to the specified project.");
            disk_types0 = disk_types0.subcommand(mcmd);
        }
        let mut disks0 = SubCommand::with_name("disks")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: add_resource_policies, aggregated_list, create_snapshot, delete, get, get_iam_policy, insert, list, remove_resource_policies, resize, set_iam_policy, set_labels and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("add_resource_policies").about("Adds existing resource policies to a disk. You can only add one policy which will be applied to this disk for scheduling snapshot creation.");
            disks0 = disks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("aggregated_list")
                .about("Retrieves an aggregated list of persistent disks.");
            disks0 = disks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create_snapshot")
                .about("Creates a snapshot of a specified persistent disk.");
            disks0 = disks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified persistent disk. Deleting a disk removes its data permanently and is irreversible. However, deleting a disk does not delete any snapshots previously made from the disk. You must separately delete snapshots.");
            disks0 = disks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns a specified persistent disk. Gets a list of available persistent disks by making a list() request.");
            disks0 = disks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. May be empty if no such policy or resource exists.");
            disks0 = disks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a persistent disk in the specified project using the data in the request. You can create a disk with a sourceImage, a sourceSnapshot, or create an empty 500 GB data disk by omitting all properties. You can also create a disk that is larger than the default size by specifying the sizeGb property.");
            disks0 = disks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a list of persistent disks contained within the specified zone.");
            disks0 = disks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("remove_resource_policies")
                .about("Removes resource policies from a disk.");
            disks0 = disks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("resize").about("Resizes the specified persistent disk. You can only increase the size of the disk.");
            disks0 = disks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy.");
            disks0 = disks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_labels").about("Sets the labels on a disk. To learn more about labels, read the Labeling Resources documentation.");
            disks0 = disks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions")
                .about("Returns permissions that a caller has on the specified resource.");
            disks0 = disks0.subcommand(mcmd);
        }
        let mut external_vpn_gateways0 = SubCommand::with_name("external_vpn_gateways")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, set_labels and test_iam_permissions");
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes the specified externalVpnGateway.");
            external_vpn_gateways0 = external_vpn_gateways0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified externalVpnGateway. Get a list of available externalVpnGateways by making a list() request.");
            external_vpn_gateways0 = external_vpn_gateways0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a ExternalVpnGateway in the specified project using the data included in the request.");
            external_vpn_gateways0 = external_vpn_gateways0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Retrieves the list of ExternalVpnGateway available to the specified project.",
            );
            external_vpn_gateways0 = external_vpn_gateways0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_labels").about("Sets the labels on an ExternalVpnGateway. To learn more about labels, read the Labeling Resources documentation.");
            external_vpn_gateways0 = external_vpn_gateways0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions")
                .about("Returns permissions that a caller has on the specified resource.");
            external_vpn_gateways0 = external_vpn_gateways0.subcommand(mcmd);
        }
        let mut firewalls0 = SubCommand::with_name("firewalls")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified firewall.");
            firewalls0 = firewalls0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified firewall.");
            firewalls0 = firewalls0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a firewall rule in the specified project using the data included in the request.");
            firewalls0 = firewalls0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves the list of firewall rules available to the specified project.");
            firewalls0 = firewalls0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified firewall rule with the data included in the request. This method supports PATCH semantics and uses the JSON merge patch format and processing rules.");
            firewalls0 = firewalls0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates the specified firewall rule with the data included in the request. Note that all fields will be updated if using PUT, even fields that are not specified. To update individual fields, please use PATCH instead.");
            firewalls0 = firewalls0.subcommand(mcmd);
        }
        let mut forwarding_rules0 = SubCommand::with_name("forwarding_rules")
            .setting(AppSettings::ColoredHelp)
            .about("methods: aggregated_list, delete, get, insert, list, patch and set_target");
        {
            let mcmd = SubCommand::with_name("aggregated_list")
                .about("Retrieves an aggregated list of forwarding rules.");
            forwarding_rules0 = forwarding_rules0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes the specified ForwardingRule resource.");
            forwarding_rules0 = forwarding_rules0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Returns the specified ForwardingRule resource.");
            forwarding_rules0 = forwarding_rules0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a ForwardingRule resource in the specified project and region using the data included in the request.");
            forwarding_rules0 = forwarding_rules0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves a list of ForwardingRule resources available to the specified project and region.");
            forwarding_rules0 = forwarding_rules0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified forwarding rule with the data included in the request. This method supports PATCH semantics and uses the JSON merge patch format and processing rules. Currently, you can only patch the network_tier field.");
            forwarding_rules0 = forwarding_rules0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_target").about("Changes target URL for forwarding rule. The new target should be of the same type as the old target.");
            forwarding_rules0 = forwarding_rules0.subcommand(mcmd);
        }
        let mut global_addresses0 = SubCommand::with_name("global_addresses")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert and list");
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes the specified address resource.");
            global_addresses0 = global_addresses0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified address resource. Gets a list of available addresses by making a list() request.");
            global_addresses0 = global_addresses0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates an address resource in the specified project by using the data included in the request.");
            global_addresses0 = global_addresses0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves a list of global addresses.");
            global_addresses0 = global_addresses0.subcommand(mcmd);
        }
        let mut global_forwarding_rules0 = SubCommand::with_name("global_forwarding_rules")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, patch and set_target");
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes the specified GlobalForwardingRule resource.");
            global_forwarding_rules0 = global_forwarding_rules0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified GlobalForwardingRule resource. Gets a list of available forwarding rules by making a list() request.");
            global_forwarding_rules0 = global_forwarding_rules0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a GlobalForwardingRule resource in the specified project using the data included in the request.");
            global_forwarding_rules0 = global_forwarding_rules0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves a list of GlobalForwardingRule resources available to the specified project.");
            global_forwarding_rules0 = global_forwarding_rules0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified forwarding rule with the data included in the request. This method supports PATCH semantics and uses the JSON merge patch format and processing rules. Currently, you can only patch the network_tier field.");
            global_forwarding_rules0 = global_forwarding_rules0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_target").about("Changes target URL for the GlobalForwardingRule resource. The new target should be of the same type as the old target.");
            global_forwarding_rules0 = global_forwarding_rules0.subcommand(mcmd);
        }
        let mut global_network_endpoint_groups0 = SubCommand::with_name("global_network_endpoint_groups")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: attach_network_endpoints, delete, detach_network_endpoints, get, insert, list and list_network_endpoints");
        {
            let mcmd = SubCommand::with_name("attach_network_endpoints")
                .about("Attach a network endpoint to the specified network endpoint group.");
            global_network_endpoint_groups0 = global_network_endpoint_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified network endpoint group.Note that the NEG cannot be deleted if there are backend services referencing it.");
            global_network_endpoint_groups0 = global_network_endpoint_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("detach_network_endpoints")
                .about("Detach the network endpoint from the specified network endpoint group.");
            global_network_endpoint_groups0 = global_network_endpoint_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified network endpoint group. Gets a list of available network endpoint groups by making a list() request.");
            global_network_endpoint_groups0 = global_network_endpoint_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a network endpoint group in the specified project using the parameters that are included in the request.");
            global_network_endpoint_groups0 = global_network_endpoint_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves the list of network endpoint groups that are located in the specified project.");
            global_network_endpoint_groups0 = global_network_endpoint_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list_network_endpoints")
                .about("Lists the network endpoints in the specified network endpoint group.");
            global_network_endpoint_groups0 = global_network_endpoint_groups0.subcommand(mcmd);
        }
        let mut global_operations0 = SubCommand::with_name("global_operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: aggregated_list, delete, get, list and wait");
        {
            let mcmd = SubCommand::with_name("aggregated_list")
                .about("Retrieves an aggregated list of all operations.");
            global_operations0 = global_operations0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes the specified Operations resource.");
            global_operations0 = global_operations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves the specified Operations resource. Gets a list of operations by making a `list()` request.");
            global_operations0 = global_operations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Retrieves a list of Operation resources contained within the specified project.",
            );
            global_operations0 = global_operations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("wait").about("Waits for the specified Operation resource to return as `DONE` or for the request to approach the 2 minute deadline, and retrieves the specified Operation resource. This method differs from the `GET` method in that it waits for no more than the default deadline (2 minutes) and then returns the current state of the operation, which might be `DONE` or still in progress.\n\nThis method is called on a best-effort basis. Specifically:  \n- In uncommon cases, when the server is overloaded, the request might return before the default deadline is reached, or might return after zero seconds. \n- If the default deadline is reached, there is no guarantee that the operation is actually done when the method returns. Be prepared to retry if the operation is not `DONE`.");
            global_operations0 = global_operations0.subcommand(mcmd);
        }
        let mut health_checks0 = SubCommand::with_name("health_checks")
            .setting(AppSettings::ColoredHelp)
            .about("methods: aggregated_list, delete, get, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("aggregated_list").about("Retrieves the list of all HealthCheck resources, regional and global, available to the specified project.");
            health_checks0 = health_checks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes the specified HealthCheck resource.");
            health_checks0 = health_checks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified HealthCheck resource. Gets a list of available health checks by making a list() request.");
            health_checks0 = health_checks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a HealthCheck resource in the specified project using the data included in the request.");
            health_checks0 = health_checks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Retrieves the list of HealthCheck resources available to the specified project.",
            );
            health_checks0 = health_checks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a HealthCheck resource in the specified project using the data included in the request. This method supports PATCH semantics and uses the JSON merge patch format and processing rules.");
            health_checks0 = health_checks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a HealthCheck resource in the specified project using the data included in the request.");
            health_checks0 = health_checks0.subcommand(mcmd);
        }
        let mut http_health_checks0 = SubCommand::with_name("http_health_checks")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes the specified HttpHealthCheck resource.");
            http_health_checks0 = http_health_checks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified HttpHealthCheck resource. Gets a list of available HTTP health checks by making a list() request.");
            http_health_checks0 = http_health_checks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a HttpHealthCheck resource in the specified project using the data included in the request.");
            http_health_checks0 = http_health_checks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves the list of HttpHealthCheck resources available to the specified project.");
            http_health_checks0 = http_health_checks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a HttpHealthCheck resource in the specified project using the data included in the request. This method supports PATCH semantics and uses the JSON merge patch format and processing rules.");
            http_health_checks0 = http_health_checks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a HttpHealthCheck resource in the specified project using the data included in the request.");
            http_health_checks0 = http_health_checks0.subcommand(mcmd);
        }
        let mut https_health_checks0 = SubCommand::with_name("https_health_checks")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes the specified HttpsHealthCheck resource.");
            https_health_checks0 = https_health_checks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified HttpsHealthCheck resource. Gets a list of available HTTPS health checks by making a list() request.");
            https_health_checks0 = https_health_checks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a HttpsHealthCheck resource in the specified project using the data included in the request.");
            https_health_checks0 = https_health_checks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves the list of HttpsHealthCheck resources available to the specified project.");
            https_health_checks0 = https_health_checks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a HttpsHealthCheck resource in the specified project using the data included in the request. This method supports PATCH semantics and uses the JSON merge patch format and processing rules.");
            https_health_checks0 = https_health_checks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a HttpsHealthCheck resource in the specified project using the data included in the request.");
            https_health_checks0 = https_health_checks0.subcommand(mcmd);
        }
        let mut images0 = SubCommand::with_name("images")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: delete, deprecate, get, get_from_family, get_iam_policy, insert, list, set_iam_policy, set_labels and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified image.");
            images0 = images0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("deprecate").about("Sets the deprecation status of an image.\n\nIf an empty request body is given, clears the deprecation status instead.");
            images0 = images0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified image. Gets a list of available images by making a list() request.");
            images0 = images0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_from_family").about(
                "Returns the latest image that is part of an image family and is not deprecated.",
            );
            images0 = images0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. May be empty if no such policy or resource exists.");
            images0 = images0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about(
                "Creates an image in the specified project using the data included in the request.",
            );
            images0 = images0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves the list of custom images available to the specified project. Custom images are images you create that belong to your project. This method does not get any images that belong to other projects, including publicly-available images, like Debian 8. If you want to get a list of publicly-available images, use this method to make a request to the respective image project, such as debian-cloud or windows-cloud.");
            images0 = images0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy.");
            images0 = images0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_labels").about("Sets the labels on an image. To learn more about labels, read the Labeling Resources documentation.");
            images0 = images0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions")
                .about("Returns permissions that a caller has on the specified resource.");
            images0 = images0.subcommand(mcmd);
        }
        let mut instance_group_managers0 = SubCommand::with_name("instance_group_managers")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: abandon_instances, aggregated_list, apply_updates_to_instances, create_instances, delete, delete_instances, get, insert, list, list_errors, list_managed_instances, patch, recreate_instances, resize, set_instance_template and set_target_pools");
        {
            let mcmd = SubCommand::with_name("abandon_instances").about("Flags the specified instances to be removed from the managed instance group. Abandoning an instance does not delete the instance, but it does remove the instance from any target pools that are applied by the managed instance group. This method reduces the targetSize of the managed instance group by the number of instances that you abandon. This operation is marked as DONE when the action is scheduled even if the instances have not yet been removed from the group. You must separately verify the status of the abandoning action with the listmanagedinstances method.\n\nIf the group is part of a backend service that has enabled connection draining, it can take up to 60 seconds after the connection draining duration has elapsed before the VM instance is removed or deleted.\n\nYou can specify a maximum of 1000 instances with this method per request.");
            instance_group_managers0 = instance_group_managers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("aggregated_list")
                .about("Retrieves the list of managed instance groups and groups them by zone.");
            instance_group_managers0 = instance_group_managers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("apply_updates_to_instances").about("Applies changes to selected instances on the managed instance group. This method can be used to apply new overrides and/or new versions.");
            instance_group_managers0 = instance_group_managers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create_instances").about("Creates instances with per-instance configs in this managed instance group. Instances are created using the current instance template. The create instances operation is marked DONE if the createInstances request is successful. The underlying actions take additional time. You must separately verify the status of the creating or actions with the listmanagedinstances method.");
            instance_group_managers0 = instance_group_managers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified managed instance group and all of the instances in that group. Note that the instance group must not belong to a backend service. Read  Deleting an instance group for more information.");
            instance_group_managers0 = instance_group_managers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete_instances").about("Flags the specified instances in the managed instance group for immediate deletion. The instances are also removed from any target pools of which they were a member. This method reduces the targetSize of the managed instance group by the number of instances that you delete. This operation is marked as DONE when the action is scheduled even if the instances are still being deleted. You must separately verify the status of the deleting action with the listmanagedinstances method.\n\nIf the group is part of a backend service that has enabled connection draining, it can take up to 60 seconds after the connection draining duration has elapsed before the VM instance is removed or deleted.\n\nYou can specify a maximum of 1000 instances with this method per request.");
            instance_group_managers0 = instance_group_managers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns all of the details about the specified managed instance group. Gets a list of available managed instance groups by making a list() request.");
            instance_group_managers0 = instance_group_managers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a managed instance group using the information that you specify in the request. After the group is created, instances in the group are created using the specified instance template. This operation is marked as DONE when the group is created even if the instances in the group have not yet been created. You must separately verify the status of the individual instances with the listmanagedinstances method.\n\nA managed instance group can have up to 1000 VM instances per group. Please contact Cloud Support if you need an increase in this limit.");
            instance_group_managers0 = instance_group_managers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves a list of managed instance groups that are contained within the specified project and zone.");
            instance_group_managers0 = instance_group_managers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list_errors").about("Lists all errors thrown by actions on instances for a given managed instance group.");
            instance_group_managers0 = instance_group_managers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list_managed_instances").about("Lists all of the instances in the managed instance group. Each instance in the list has a currentAction, which indicates the action that the managed instance group is performing on the instance. For example, if the group is still creating an instance, the currentAction is CREATING. If a previous action failed, the list displays the errors for that failed action.");
            instance_group_managers0 = instance_group_managers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a managed instance group using the information that you specify in the request. This operation is marked as DONE when the group is patched even if the instances in the group are still in the process of being patched. You must separately verify the status of the individual instances with the listManagedInstances method. This method supports PATCH semantics and uses the JSON merge patch format and processing rules.");
            instance_group_managers0 = instance_group_managers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("recreate_instances").about("Flags the specified instances in the managed instance group to be immediately recreated. The instances are deleted and recreated using the current instance template for the managed instance group. This operation is marked as DONE when the flag is set even if the instances have not yet been recreated. You must separately verify the status of the recreating action with the listmanagedinstances method.\n\nIf the group is part of a backend service that has enabled connection draining, it can take up to 60 seconds after the connection draining duration has elapsed before the VM instance is removed or deleted.\n\nYou can specify a maximum of 1000 instances with this method per request.");
            instance_group_managers0 = instance_group_managers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("resize").about("Resizes the managed instance group. If you increase the size, the group creates new instances using the current instance template. If you decrease the size, the group deletes instances. The resize operation is marked DONE when the resize actions are scheduled even if the group has not yet added or deleted any instances. You must separately verify the status of the creating or deleting actions with the listmanagedinstances method.\n\nWhen resizing down, the instance group arbitrarily chooses the order in which VMs are deleted. The group takes into account some VM attributes when making the selection including:\n\n+ The status of the VM instance. + The health of the VM instance. + The instance template version the VM is based on. + For regional managed instance groups, the location of the VM instance.\n\nThis list is subject to change.\n\nIf the group is part of a backend service that has enabled connection draining, it can take up to 60 seconds after the connection draining duration has elapsed before the VM instance is removed or deleted.");
            instance_group_managers0 = instance_group_managers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_instance_template").about("Specifies the instance template to use when creating new instances in this group. The templates for existing instances in the group do not change unless you recreate them.");
            instance_group_managers0 = instance_group_managers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_target_pools").about("Modifies the target pools to which all instances in this managed instance group are assigned. The target pools automatically apply to all of the instances in the managed instance group. This operation is marked DONE when you make the request even if the instances have not yet been added to their target pools. The change might take some time to apply to all of the instances in the group depending on the size of the group.");
            instance_group_managers0 = instance_group_managers0.subcommand(mcmd);
        }
        let mut instance_groups0 = SubCommand::with_name("instance_groups")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: add_instances, aggregated_list, delete, get, insert, list, list_instances, remove_instances and set_named_ports");
        {
            let mcmd = SubCommand::with_name("add_instances").about("Adds a list of instances to the specified instance group. All of the instances in the instance group must be in the same network/subnetwork. Read  Adding instances for more information.");
            instance_groups0 = instance_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("aggregated_list")
                .about("Retrieves the list of instance groups and sorts them by zone.");
            instance_groups0 = instance_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified instance group. The instances in the group are not deleted. Note that instance group must not belong to a backend service. Read  Deleting an instance group for more information.");
            instance_groups0 = instance_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified instance group. Gets a list of available instance groups by making a list() request.");
            instance_groups0 = instance_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates an instance group in the specified project using the parameters that are included in the request.");
            instance_groups0 = instance_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves the list of instance groups that are located in the specified project and zone.");
            instance_groups0 = instance_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list_instances")
                .about("Lists the instances in the specified instance group.");
            instance_groups0 = instance_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("remove_instances").about("Removes one or more instances from the specified instance group, but does not delete those instances.\n\nIf the group is part of a backend service that has enabled connection draining, it can take up to 60 seconds after the connection draining duration before the VM instance is removed or deleted.");
            instance_groups0 = instance_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_named_ports")
                .about("Sets the named ports for the specified instance group.");
            instance_groups0 = instance_groups0.subcommand(mcmd);
        }
        let mut instance_templates0 = SubCommand::with_name("instance_templates")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: delete, get, get_iam_policy, insert, list, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified instance template. Deleting an instance template is permanent and cannot be undone. It is not possible to delete templates that are already in use by a managed instance group.");
            instance_templates0 = instance_templates0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified instance template. Gets a list of available instance templates by making a list() request.");
            instance_templates0 = instance_templates0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. May be empty if no such policy or resource exists.");
            instance_templates0 = instance_templates0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates an instance template in the specified project using the data that is included in the request. If you are creating a new template to update an existing instance group, your new instance template must use the same network or, if applicable, the same subnetwork as the original template.");
            instance_templates0 = instance_templates0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves a list of instance templates that are contained within the specified project.");
            instance_templates0 = instance_templates0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy.");
            instance_templates0 = instance_templates0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions")
                .about("Returns permissions that a caller has on the specified resource.");
            instance_templates0 = instance_templates0.subcommand(mcmd);
        }
        let mut instances0 = SubCommand::with_name("instances")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: add_access_config, add_resource_policies, aggregated_list, attach_disk, delete, delete_access_config, detach_disk, get, get_guest_attributes, get_iam_policy, get_serial_port_output, get_shielded_instance_identity, insert, list, list_referrers, remove_resource_policies, reset, set_deletion_protection, set_disk_auto_delete, set_iam_policy, set_labels, set_machine_resources, set_machine_type, set_metadata, set_min_cpu_platform, set_scheduling, set_service_account, set_shielded_instance_integrity_policy, set_tags, simulate_maintenance_event, start, start_with_encryption_key, stop, test_iam_permissions, update, update_access_config, update_display_device, update_network_interface and update_shielded_instance_config");
        {
            let mcmd = SubCommand::with_name("add_access_config")
                .about("Adds an access config to an instance\'s network interface.");
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("add_resource_policies").about("Adds existing resource policies to an instance. You can only add one policy right now which will be applied to this instance for scheduling live migrations.");
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("aggregated_list").about("Retrieves aggregated list of all of the instances in your project across all regions and zones.");
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("attach_disk").about("Attaches an existing Disk resource to an instance. You must first create the disk before you can attach it. It is not possible to create and attach a disk at the same time. For more information, read Adding a persistent disk to your instance.");
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified Instance resource. For more information, see Stopping or Deleting an Instance.");
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete_access_config")
                .about("Deletes an access config from an instance\'s network interface.");
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("detach_disk").about("Detaches a disk from an instance.");
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified Instance resource. Gets a list of available instances by making a list() request.");
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_guest_attributes")
                .about("Returns the specified guest attributes entry.");
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. May be empty if no such policy or resource exists.");
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_serial_port_output")
                .about("Returns the last 1 MB of serial port output from the specified instance.");
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_shielded_instance_identity")
                .about("Returns the Shielded Instance Identity of an instance");
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates an instance resource in the specified project using the data included in the request.");
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves the list of instances contained within the specified zone.");
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list_referrers").about("Retrieves a list of resources that refer to the VM instance specified in the request. For example, if the VM instance is part of a managed instance group, the referrers list includes the managed instance group. For more information, read Viewing Referrers to VM Instances.");
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("remove_resource_policies")
                .about("Removes resource policies from an instance.");
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("reset").about("Performs a reset on the instance. This is a hard reset the VM does not do a graceful shutdown. For more information, see Resetting an instance.");
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_deletion_protection")
                .about("Sets deletion protection on the instance.");
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_disk_auto_delete")
                .about("Sets the auto-delete flag for a disk attached to an instance.");
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy.");
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_labels").about("Sets labels on an instance. To learn more about labels, read the Labeling Resources documentation.");
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_machine_resources").about("Changes the number and/or type of accelerator for a stopped instance to the values specified in the request.");
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_machine_type").about("Changes the machine type for a stopped instance to the machine type specified in the request.");
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_metadata").about(
                "Sets metadata for the specified instance to the data included in the request.",
            );
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_min_cpu_platform").about("Changes the minimum CPU platform that this instance should use. This method can only be called on a stopped instance. For more information, read Specifying a Minimum CPU Platform.");
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_scheduling").about("Sets an instance\'s scheduling options. You can only call this method on a stopped instance, that is, a VM instance that is in a `TERMINATED` state. See Instance Life Cycle for more information on the possible instance states.");
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_service_account").about("Sets the service account on the instance. For more information, read Changing the service account and access scopes for an instance.");
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_shielded_instance_integrity_policy").about("Sets the Shielded Instance integrity policy for an instance. You can only use this method on a running instance. This method supports PATCH semantics and uses the JSON merge patch format and processing rules.");
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_tags").about(
                "Sets network tags for the specified instance to the data included in the request.",
            );
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("simulate_maintenance_event")
                .about("Simulates a maintenance event on the instance.");
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("start").about("Starts an instance that was stopped using the instances().stop method. For more information, see Restart an instance.");
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("start_with_encryption_key").about("Starts an instance that was stopped using the instances().stop method. For more information, see Restart an instance.");
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("stop").about("Stops a running instance, shutting it down cleanly, and allows you to restart the instance at a later time. Stopped instances do not incur VM usage charges while they are stopped. However, resources that the VM is using, such as persistent disks and static IP addresses, will continue to be charged until they are deleted. For more information, see Stopping an instance.");
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions")
                .about("Returns permissions that a caller has on the specified resource.");
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates an instance only if the necessary resources are available. This method can update only a specific set of instance properties. See  Updating a running instance for a list of updatable instance properties.");
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_access_config").about("Updates the specified access config from an instance\'s network interface with the data included in the request. This method supports PATCH semantics and uses the JSON merge patch format and processing rules.");
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_display_device").about("Updates the Display config for a VM instance. You can only use this method on a stopped VM instance. This method supports PATCH semantics and uses the JSON merge patch format and processing rules.");
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_network_interface").about(
                "Updates an instance\'s network interface. This method follows PATCH semantics.",
            );
            instances0 = instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_shielded_instance_config").about("Updates the Shielded Instance config for an instance. You can only use this method on a stopped instance. This method supports PATCH semantics and uses the JSON merge patch format and processing rules.");
            instances0 = instances0.subcommand(mcmd);
        }
        let mut interconnect_attachments0 = SubCommand::with_name("interconnect_attachments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: aggregated_list, delete, get, insert, list and patch");
        {
            let mcmd = SubCommand::with_name("aggregated_list")
                .about("Retrieves an aggregated list of interconnect attachments.");
            interconnect_attachments0 = interconnect_attachments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes the specified interconnect attachment.");
            interconnect_attachments0 = interconnect_attachments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Returns the specified interconnect attachment.");
            interconnect_attachments0 = interconnect_attachments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates an InterconnectAttachment in the specified project using the data included in the request.");
            interconnect_attachments0 = interconnect_attachments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves the list of interconnect attachments contained within the specified region.");
            interconnect_attachments0 = interconnect_attachments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified interconnect attachment with the data included in the request. This method supports PATCH semantics and uses the JSON merge patch format and processing rules.");
            interconnect_attachments0 = interconnect_attachments0.subcommand(mcmd);
        }
        let mut interconnect_locations0 = SubCommand::with_name("interconnect_locations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Returns the details for the specified interconnect location. Gets a list of available interconnect locations by making a list() request.");
            interconnect_locations0 = interconnect_locations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Retrieves the list of interconnect locations available to the specified project.",
            );
            interconnect_locations0 = interconnect_locations0.subcommand(mcmd);
        }
        let mut interconnects0 = SubCommand::with_name("interconnects")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, get_diagnostics, insert, list and patch");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified interconnect.");
            interconnects0 = interconnects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified interconnect. Get a list of available interconnects by making a list() request.");
            interconnects0 = interconnects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_diagnostics")
                .about("Returns the interconnectDiagnostics for the specified interconnect.");
            interconnects0 = interconnects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a Interconnect in the specified project using the data included in the request.");
            interconnects0 = interconnects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves the list of interconnect available to the specified project.");
            interconnects0 = interconnects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified interconnect with the data included in the request. This method supports PATCH semantics and uses the JSON merge patch format and processing rules.");
            interconnects0 = interconnects0.subcommand(mcmd);
        }
        let mut license_codes0 = SubCommand::with_name("license_codes")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("get").about("Return a specified license code. License codes are mirrored across all projects that have permissions to read the License Code.  Caution This resource is intended for use only by third-party partners who are creating Cloud Marketplace images.");
            license_codes0 = license_codes0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource.  Caution This resource is intended for use only by third-party partners who are creating Cloud Marketplace images.");
            license_codes0 = license_codes0.subcommand(mcmd);
        }
        let mut licenses0 = SubCommand::with_name("licenses")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: delete, get, get_iam_policy, insert, list, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified license.  Caution This resource is intended for use only by third-party partners who are creating Cloud Marketplace images.");
            licenses0 = licenses0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified License resource.  Caution This resource is intended for use only by third-party partners who are creating Cloud Marketplace images.");
            licenses0 = licenses0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. May be empty if no such policy or resource exists.  Caution This resource is intended for use only by third-party partners who are creating Cloud Marketplace images.");
            licenses0 = licenses0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Create a License resource in the specified project.  Caution This resource is intended for use only by third-party partners who are creating Cloud Marketplace images.");
            licenses0 = licenses0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves the list of licenses available in the specified project. This method does not get any licenses that belong to other projects, including licenses attached to publicly-available images, like Debian 9. If you want to get a list of publicly-available licenses, use this method to make a request to the respective image project, such as debian-cloud or windows-cloud.  Caution This resource is intended for use only by third-party partners who are creating Cloud Marketplace images.");
            licenses0 = licenses0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy.  Caution This resource is intended for use only by third-party partners who are creating Cloud Marketplace images.");
            licenses0 = licenses0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource.  Caution This resource is intended for use only by third-party partners who are creating Cloud Marketplace images.");
            licenses0 = licenses0.subcommand(mcmd);
        }
        let mut machine_types0 = SubCommand::with_name("machine_types")
            .setting(AppSettings::ColoredHelp)
            .about("methods: aggregated_list, get and list");
        {
            let mcmd = SubCommand::with_name("aggregated_list")
                .about("Retrieves an aggregated list of machine types.");
            machine_types0 = machine_types0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified machine type. Gets a list of available machine types by making a list() request.");
            machine_types0 = machine_types0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a list of machine types available to the specified project.");
            machine_types0 = machine_types0.subcommand(mcmd);
        }
        let mut network_endpoint_groups0 = SubCommand::with_name("network_endpoint_groups")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: aggregated_list, attach_network_endpoints, delete, detach_network_endpoints, get, insert, list, list_network_endpoints and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("aggregated_list")
                .about("Retrieves the list of network endpoint groups and sorts them by zone.");
            network_endpoint_groups0 = network_endpoint_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("attach_network_endpoints").about(
                "Attach a list of network endpoints to the specified network endpoint group.",
            );
            network_endpoint_groups0 = network_endpoint_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified network endpoint group. The network endpoints in the NEG and the VM instances they belong to are not terminated when the NEG is deleted. Note that the NEG cannot be deleted if there are backend services referencing it.");
            network_endpoint_groups0 = network_endpoint_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("detach_network_endpoints").about(
                "Detach a list of network endpoints from the specified network endpoint group.",
            );
            network_endpoint_groups0 = network_endpoint_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified network endpoint group. Gets a list of available network endpoint groups by making a list() request.");
            network_endpoint_groups0 = network_endpoint_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a network endpoint group in the specified project using the parameters that are included in the request.");
            network_endpoint_groups0 = network_endpoint_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves the list of network endpoint groups that are located in the specified project and zone.");
            network_endpoint_groups0 = network_endpoint_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list_network_endpoints")
                .about("Lists the network endpoints in the specified network endpoint group.");
            network_endpoint_groups0 = network_endpoint_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions")
                .about("Returns permissions that a caller has on the specified resource.");
            network_endpoint_groups0 = network_endpoint_groups0.subcommand(mcmd);
        }
        let mut networks0 = SubCommand::with_name("networks")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: add_peering, delete, get, insert, list, list_peering_routes, patch, remove_peering, switch_to_custom_mode and update_peering");
        {
            let mcmd = SubCommand::with_name("add_peering")
                .about("Adds a peering to the specified network.");
            networks0 = networks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified network.");
            networks0 = networks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified network. Gets a list of available networks by making a list() request.");
            networks0 = networks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a network in the specified project using the data included in the request.");
            networks0 = networks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves the list of networks available to the specified project.");
            networks0 = networks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list_peering_routes")
                .about("Lists the peering routes exchanged over peering connection.");
            networks0 = networks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Patches the specified network with the data included in the request. Only the following fields can be modified: routingConfig.routingMode.");
            networks0 = networks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("remove_peering")
                .about("Removes a peering from the specified network.");
            networks0 = networks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("switch_to_custom_mode")
                .about("Switches the network mode from auto subnet mode to custom subnet mode.");
            networks0 = networks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_peering").about("Updates the specified network peering with the data included in the request Only the following fields can be modified: NetworkPeering.export_custom_routes, and NetworkPeering.import_custom_routes");
            networks0 = networks0.subcommand(mcmd);
        }
        let mut node_groups0 = SubCommand::with_name("node_groups")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: add_nodes, aggregated_list, delete, delete_nodes, get, get_iam_policy, insert, list, list_nodes, patch, set_iam_policy, set_node_template and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("add_nodes")
                .about("Adds specified number of nodes to the node group.");
            node_groups0 = node_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("aggregated_list").about("Retrieves an aggregated list of node groups. Note: use nodeGroups.listNodes for more details about each group.");
            node_groups0 = node_groups0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes the specified NodeGroup resource.");
            node_groups0 = node_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete_nodes")
                .about("Deletes specified nodes from the node group.");
            node_groups0 = node_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified NodeGroup. Get a list of available NodeGroups by making a list() request. Note: the \"nodes\" field should not be used. Use nodeGroups.listNodes instead.");
            node_groups0 = node_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. May be empty if no such policy or resource exists.");
            node_groups0 = node_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a NodeGroup resource in the specified project using the data included in the request.");
            node_groups0 = node_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves a list of node groups available to the specified project. Note: use nodeGroups.listNodes for more details about each group.");
            node_groups0 = node_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list_nodes").about("Lists nodes in the node group.");
            node_groups0 = node_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified node group.");
            node_groups0 = node_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy.");
            node_groups0 = node_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_node_template")
                .about("Updates the node template of the node group.");
            node_groups0 = node_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions")
                .about("Returns permissions that a caller has on the specified resource.");
            node_groups0 = node_groups0.subcommand(mcmd);
        }
        let mut node_templates0 = SubCommand::with_name("node_templates")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: aggregated_list, delete, get, get_iam_policy, insert, list, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("aggregated_list")
                .about("Retrieves an aggregated list of node templates.");
            node_templates0 = node_templates0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes the specified NodeTemplate resource.");
            node_templates0 = node_templates0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified node template. Gets a list of available node templates by making a list() request.");
            node_templates0 = node_templates0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. May be empty if no such policy or resource exists.");
            node_templates0 = node_templates0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a NodeTemplate resource in the specified project using the data included in the request.");
            node_templates0 = node_templates0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a list of node templates available to the specified project.");
            node_templates0 = node_templates0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy.");
            node_templates0 = node_templates0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions")
                .about("Returns permissions that a caller has on the specified resource.");
            node_templates0 = node_templates0.subcommand(mcmd);
        }
        let mut node_types0 = SubCommand::with_name("node_types")
            .setting(AppSettings::ColoredHelp)
            .about("methods: aggregated_list, get and list");
        {
            let mcmd = SubCommand::with_name("aggregated_list")
                .about("Retrieves an aggregated list of node types.");
            node_types0 = node_types0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified node type. Gets a list of available node types by making a list() request.");
            node_types0 = node_types0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a list of node types available to the specified project.");
            node_types0 = node_types0.subcommand(mcmd);
        }
        let mut packet_mirrorings0 = SubCommand::with_name("packet_mirrorings")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: aggregated_list, delete, get, insert, list, patch and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("aggregated_list")
                .about("Retrieves an aggregated list of packetMirrorings.");
            packet_mirrorings0 = packet_mirrorings0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes the specified PacketMirroring resource.");
            packet_mirrorings0 = packet_mirrorings0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Returns the specified PacketMirroring resource.");
            packet_mirrorings0 = packet_mirrorings0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a PacketMirroring resource in the specified project and region using the data included in the request.");
            packet_mirrorings0 = packet_mirrorings0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves a list of PacketMirroring resources available to the specified project and region.");
            packet_mirrorings0 = packet_mirrorings0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Patches the specified PacketMirroring resource with the data included in the request. This method supports PATCH semantics and uses JSON merge patch format and processing rules.");
            packet_mirrorings0 = packet_mirrorings0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions")
                .about("Returns permissions that a caller has on the specified resource.");
            packet_mirrorings0 = packet_mirrorings0.subcommand(mcmd);
        }
        let mut projects0 = SubCommand::with_name("projects")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: disable_xpn_host, disable_xpn_resource, enable_xpn_host, enable_xpn_resource, get, get_xpn_host, get_xpn_resources, list_xpn_hosts, move_disk, move_instance, set_common_instance_metadata, set_default_network_tier and set_usage_export_bucket");
        {
            let mcmd = SubCommand::with_name("disable_xpn_host")
                .about("Disable this project as a shared VPC host project.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("disable_xpn_resource").about("Disable a service resource (also known as service project) associated with this host project.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("enable_xpn_host")
                .about("Enable this project as a shared VPC host project.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("enable_xpn_resource").about("Enable service resource (a.k.a service project) for a host project, so that subnets in the host project can be used by instances in the service project.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Returns the specified Project resource.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_xpn_host").about("Gets the shared VPC host project that this project links to. May be empty if no link exists.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_xpn_resources").about(
                "Gets service resources (a.k.a service project) associated with this host project.",
            );
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list_xpn_hosts").about(
                "Lists all shared VPC host projects visible to the user in an organization.",
            );
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("move_disk")
                .about("Moves a persistent disk from one zone to another.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("move_instance").about(
                "Moves an instance and its attached persistent disks from one zone to another.",
            );
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_common_instance_metadata").about("Sets metadata common to all instances within the specified project using the data included in the request.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_default_network_tier").about("Sets the default network tier of the project. The default network tier is used when an address/forwardingRule/instance is created without specifying the network tier field.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_usage_export_bucket").about("Enables the usage export feature and sets the usage export bucket where reports are stored. If you provide an empty request body using this method, the usage export feature will be disabled.");
            projects0 = projects0.subcommand(mcmd);
        }
        let mut region_autoscalers0 = SubCommand::with_name("region_autoscalers")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified autoscaler.");
            region_autoscalers0 = region_autoscalers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified autoscaler.");
            region_autoscalers0 = region_autoscalers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates an autoscaler in the specified project using the data included in the request.");
            region_autoscalers0 = region_autoscalers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a list of autoscalers contained within the specified region.");
            region_autoscalers0 = region_autoscalers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an autoscaler in the specified project using the data included in the request. This method supports PATCH semantics and uses the JSON merge patch format and processing rules.");
            region_autoscalers0 = region_autoscalers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates an autoscaler in the specified project using the data included in the request.");
            region_autoscalers0 = region_autoscalers0.subcommand(mcmd);
        }
        let mut region_backend_services0 = SubCommand::with_name("region_backend_services")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, get_health, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes the specified regional BackendService resource.");
            region_backend_services0 = region_backend_services0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Returns the specified regional BackendService resource.");
            region_backend_services0 = region_backend_services0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_health").about(
                "Gets the most recent health check results for this regional BackendService.",
            );
            region_backend_services0 = region_backend_services0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a regional BackendService resource in the specified project using the data included in the request. There are several restrictions and guidelines to keep in mind when creating a regional backend service. Read  Understanding backend services for more information.");
            region_backend_services0 = region_backend_services0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves the list of regional BackendService resources available to the specified project in the given region.");
            region_backend_services0 = region_backend_services0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified regional BackendService resource with the data included in the request. There are several Understanding backend services to keep in mind when updating a backend service. Read  Understanding backend services for more information. This method supports PATCH semantics and uses the JSON merge patch format and processing rules.");
            region_backend_services0 = region_backend_services0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates the specified regional BackendService resource with the data included in the request. There are several Understanding backend services to keep in mind when updating a backend service. Read  Understanding backend services for more information.");
            region_backend_services0 = region_backend_services0.subcommand(mcmd);
        }
        let mut region_commitments0 = SubCommand::with_name("region_commitments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: aggregated_list, get, insert and list");
        {
            let mcmd = SubCommand::with_name("aggregated_list")
                .about("Retrieves an aggregated list of commitments.");
            region_commitments0 = region_commitments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified commitment resource. Gets a list of available commitments by making a list() request.");
            region_commitments0 = region_commitments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a commitment in the specified project using the data included in the request.");
            region_commitments0 = region_commitments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a list of commitments contained within the specified region.");
            region_commitments0 = region_commitments0.subcommand(mcmd);
        }
        let mut region_disk_types0 = SubCommand::with_name("region_disk_types")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified regional disk type. Gets a list of available disk types by making a list() request.");
            region_disk_types0 = region_disk_types0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Retrieves a list of regional disk types available to the specified project.",
            );
            region_disk_types0 = region_disk_types0.subcommand(mcmd);
        }
        let mut region_disks0 = SubCommand::with_name("region_disks")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: add_resource_policies, create_snapshot, delete, get, get_iam_policy, insert, list, remove_resource_policies, resize, set_iam_policy, set_labels and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("add_resource_policies").about("Adds existing resource policies to a regional disk. You can only add one policy which will be applied to this disk for scheduling snapshot creation.");
            region_disks0 = region_disks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create_snapshot")
                .about("Creates a snapshot of this regional disk.");
            region_disks0 = region_disks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified regional persistent disk. Deleting a regional disk removes all the replicas of its data permanently and is irreversible. However, deleting a disk does not delete any snapshots previously made from the disk. You must separately delete snapshots.");
            region_disks0 = region_disks0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Returns a specified regional persistent disk.");
            region_disks0 = region_disks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. May be empty if no such policy or resource exists.");
            region_disks0 = region_disks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a persistent regional disk in the specified project using the data included in the request.");
            region_disks0 = region_disks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Retrieves the list of persistent disks contained within the specified region.",
            );
            region_disks0 = region_disks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("remove_resource_policies")
                .about("Removes resource policies from a regional disk.");
            region_disks0 = region_disks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("resize")
                .about("Resizes the specified regional persistent disk.");
            region_disks0 = region_disks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy.");
            region_disks0 = region_disks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_labels")
                .about("Sets the labels on the target regional disk.");
            region_disks0 = region_disks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions")
                .about("Returns permissions that a caller has on the specified resource.");
            region_disks0 = region_disks0.subcommand(mcmd);
        }
        let mut region_health_check_services0 =
            SubCommand::with_name("region_health_check_services")
                .setting(AppSettings::ColoredHelp)
                .about("methods: delete, get, insert, list and patch");
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes the specified regional HealthCheckService.");
            region_health_check_services0 = region_health_check_services0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Returns the specified regional HealthCheckService resource.");
            region_health_check_services0 = region_health_check_services0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a regional HealthCheckService resource in the specified project and region using the data included in the request.");
            region_health_check_services0 = region_health_check_services0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all the HealthCheckService resources that have been configured for the specified project in the given region.");
            region_health_check_services0 = region_health_check_services0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified regional HealthCheckService resource with the data included in the request. This method supports PATCH semantics and uses the JSON merge patch format and processing rules.");
            region_health_check_services0 = region_health_check_services0.subcommand(mcmd);
        }
        let mut region_health_checks0 = SubCommand::with_name("region_health_checks")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes the specified HealthCheck resource.");
            region_health_checks0 = region_health_checks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified HealthCheck resource. Gets a list of available health checks by making a list() request.");
            region_health_checks0 = region_health_checks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a HealthCheck resource in the specified project using the data included in the request.");
            region_health_checks0 = region_health_checks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Retrieves the list of HealthCheck resources available to the specified project.",
            );
            region_health_checks0 = region_health_checks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a HealthCheck resource in the specified project using the data included in the request. This method supports PATCH semantics and uses the JSON merge patch format and processing rules.");
            region_health_checks0 = region_health_checks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a HealthCheck resource in the specified project using the data included in the request.");
            region_health_checks0 = region_health_checks0.subcommand(mcmd);
        }
        let mut region_instance_group_managers0 = SubCommand::with_name("region_instance_group_managers")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: abandon_instances, apply_updates_to_instances, create_instances, delete, delete_instances, get, insert, list, list_errors, list_managed_instances, patch, recreate_instances, resize, set_instance_template and set_target_pools");
        {
            let mcmd = SubCommand::with_name("abandon_instances").about("Flags the specified instances to be immediately removed from the managed instance group. Abandoning an instance does not delete the instance, but it does remove the instance from any target pools that are applied by the managed instance group. This method reduces the targetSize of the managed instance group by the number of instances that you abandon. This operation is marked as DONE when the action is scheduled even if the instances have not yet been removed from the group. You must separately verify the status of the abandoning action with the listmanagedinstances method.\n\nIf the group is part of a backend service that has enabled connection draining, it can take up to 60 seconds after the connection draining duration has elapsed before the VM instance is removed or deleted.\n\nYou can specify a maximum of 1000 instances with this method per request.");
            region_instance_group_managers0 = region_instance_group_managers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("apply_updates_to_instances")
                .about("Apply updates to selected instances the managed instance group.");
            region_instance_group_managers0 = region_instance_group_managers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create_instances").about("Creates instances with per-instance configs in this regional managed instance group. Instances are created using the current instance template. The create instances operation is marked DONE if the createInstances request is successful. The underlying actions take additional time. You must separately verify the status of the creating or actions with the listmanagedinstances method.");
            region_instance_group_managers0 = region_instance_group_managers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified managed instance group and all of the instances in that group.");
            region_instance_group_managers0 = region_instance_group_managers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete_instances").about("Flags the specified instances in the managed instance group to be immediately deleted. The instances are also removed from any target pools of which they were a member. This method reduces the targetSize of the managed instance group by the number of instances that you delete. The deleteInstances operation is marked DONE if the deleteInstances request is successful. The underlying actions take additional time. You must separately verify the status of the deleting action with the listmanagedinstances method.\n\nIf the group is part of a backend service that has enabled connection draining, it can take up to 60 seconds after the connection draining duration has elapsed before the VM instance is removed or deleted.\n\nYou can specify a maximum of 1000 instances with this method per request.");
            region_instance_group_managers0 = region_instance_group_managers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Returns all of the details about the specified managed instance group.");
            region_instance_group_managers0 = region_instance_group_managers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a managed instance group using the information that you specify in the request. After the group is created, instances in the group are created using the specified instance template. This operation is marked as DONE when the group is created even if the instances in the group have not yet been created. You must separately verify the status of the individual instances with the listmanagedinstances method.\n\nA regional managed instance group can contain up to 2000 instances.");
            region_instance_group_managers0 = region_instance_group_managers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves the list of managed instance groups that are contained within the specified region.");
            region_instance_group_managers0 = region_instance_group_managers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list_errors").about("Lists all errors thrown by actions on instances for a given regional managed instance group.");
            region_instance_group_managers0 = region_instance_group_managers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list_managed_instances").about("Lists the instances in the managed instance group and instances that are scheduled to be created. The list includes any current actions that the group has scheduled for its instances.");
            region_instance_group_managers0 = region_instance_group_managers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a managed instance group using the information that you specify in the request. This operation is marked as DONE when the group is patched even if the instances in the group are still in the process of being patched. You must separately verify the status of the individual instances with the listmanagedinstances method. This method supports PATCH semantics and uses the JSON merge patch format and processing rules.");
            region_instance_group_managers0 = region_instance_group_managers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("recreate_instances").about("Flags the specified instances in the managed instance group to be immediately recreated. The instances are deleted and recreated using the current instance template for the managed instance group. This operation is marked as DONE when the flag is set even if the instances have not yet been recreated. You must separately verify the status of the recreating action with the listmanagedinstances method.\n\nIf the group is part of a backend service that has enabled connection draining, it can take up to 60 seconds after the connection draining duration has elapsed before the VM instance is removed or deleted.\n\nYou can specify a maximum of 1000 instances with this method per request.");
            region_instance_group_managers0 = region_instance_group_managers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("resize").about("Changes the intended size of the managed instance group. If you increase the size, the group creates new instances using the current instance template. If you decrease the size, the group deletes one or more instances.\n\nThe resize operation is marked DONE if the resize request is successful. The underlying actions take additional time. You must separately verify the status of the creating or deleting actions with the listmanagedinstances method.\n\nIf the group is part of a backend service that has enabled connection draining, it can take up to 60 seconds after the connection draining duration has elapsed before the VM instance is removed or deleted.");
            region_instance_group_managers0 = region_instance_group_managers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_instance_template").about("Sets the instance template to use when creating new instances or recreating instances in this group. Existing instances are not affected.");
            region_instance_group_managers0 = region_instance_group_managers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_target_pools").about("Modifies the target pools to which all new instances in this group are assigned. Existing instances in the group are not affected.");
            region_instance_group_managers0 = region_instance_group_managers0.subcommand(mcmd);
        }
        let mut region_instance_groups0 = SubCommand::with_name("region_instance_groups")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, list, list_instances and set_named_ports");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Returns the specified instance group resource.");
            region_instance_groups0 = region_instance_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves the list of instance group resources contained within the specified region.");
            region_instance_groups0 = region_instance_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list_instances").about("Lists the instances in the specified instance group and displays information about the named ports. Depending on the specified options, this method can list all instances or only the instances that are running.");
            region_instance_groups0 = region_instance_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_named_ports")
                .about("Sets the named ports for the specified regional instance group.");
            region_instance_groups0 = region_instance_groups0.subcommand(mcmd);
        }
        let mut region_notification_endpoints0 =
            SubCommand::with_name("region_notification_endpoints")
                .setting(AppSettings::ColoredHelp)
                .about("methods: delete, get, insert and list");
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes the specified NotificationEndpoint in the given region");
            region_notification_endpoints0 = region_notification_endpoints0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Returns the specified NotificationEndpoint resource in the given region.");
            region_notification_endpoints0 = region_notification_endpoints0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Create a NotificationEndpoint in the specified project in the given region using the parameters that are included in the request.");
            region_notification_endpoints0 = region_notification_endpoints0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists the NotificationEndpoints for a project in the given region.");
            region_notification_endpoints0 = region_notification_endpoints0.subcommand(mcmd);
        }
        let mut region_operations0 = SubCommand::with_name("region_operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, list and wait");
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes the specified region-specific Operations resource.");
            region_operations0 = region_operations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Retrieves the specified region-specific Operations resource.");
            region_operations0 = region_operations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Retrieves a list of Operation resources contained within the specified region.",
            );
            region_operations0 = region_operations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("wait").about("Waits for the specified Operation resource to return as `DONE` or for the request to approach the 2 minute deadline, and retrieves the specified Operation resource. This method differs from the `GET` method in that it waits for no more than the default deadline (2 minutes) and then returns the current state of the operation, which might be `DONE` or still in progress.\n\nThis method is called on a best-effort basis. Specifically:  \n- In uncommon cases, when the server is overloaded, the request might return before the default deadline is reached, or might return after zero seconds. \n- If the default deadline is reached, there is no guarantee that the operation is actually done when the method returns. Be prepared to retry if the operation is not `DONE`.");
            region_operations0 = region_operations0.subcommand(mcmd);
        }
        let mut region_ssl_certificates0 = SubCommand::with_name("region_ssl_certificates")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert and list");
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes the specified SslCertificate resource in the region.");
            region_ssl_certificates0 = region_ssl_certificates0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified SslCertificate resource in the specified region. Get a list of available SSL certificates by making a list() request.");
            region_ssl_certificates0 = region_ssl_certificates0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a SslCertificate resource in the specified project and region using the data included in the request");
            region_ssl_certificates0 = region_ssl_certificates0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves the list of SslCertificate resources available to the specified project in the specified region.");
            region_ssl_certificates0 = region_ssl_certificates0.subcommand(mcmd);
        }
        let mut region_target_http_proxies0 = SubCommand::with_name("region_target_http_proxies")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list and set_url_map");
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes the specified TargetHttpProxy resource.");
            region_target_http_proxies0 = region_target_http_proxies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified TargetHttpProxy resource in the specified region. Gets a list of available target HTTP proxies by making a list() request.");
            region_target_http_proxies0 = region_target_http_proxies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a TargetHttpProxy resource in the specified project and region using the data included in the request.");
            region_target_http_proxies0 = region_target_http_proxies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves the list of TargetHttpProxy resources available to the specified project in the specified region.");
            region_target_http_proxies0 = region_target_http_proxies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_url_map")
                .about("Changes the URL map for TargetHttpProxy.");
            region_target_http_proxies0 = region_target_http_proxies0.subcommand(mcmd);
        }
        let mut region_target_https_proxies0 = SubCommand::with_name("region_target_https_proxies")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, set_ssl_certificates and set_url_map");
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes the specified TargetHttpsProxy resource.");
            region_target_https_proxies0 = region_target_https_proxies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified TargetHttpsProxy resource in the specified region. Gets a list of available target HTTP proxies by making a list() request.");
            region_target_https_proxies0 = region_target_https_proxies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a TargetHttpsProxy resource in the specified project and region using the data included in the request.");
            region_target_https_proxies0 = region_target_https_proxies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves the list of TargetHttpsProxy resources available to the specified project in the specified region.");
            region_target_https_proxies0 = region_target_https_proxies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_ssl_certificates")
                .about("Replaces SslCertificates for TargetHttpsProxy.");
            region_target_https_proxies0 = region_target_https_proxies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_url_map")
                .about("Changes the URL map for TargetHttpsProxy.");
            region_target_https_proxies0 = region_target_https_proxies0.subcommand(mcmd);
        }
        let mut region_url_maps0 = SubCommand::with_name("region_url_maps")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, patch, update and validate");
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes the specified UrlMap resource.");
            region_url_maps0 = region_url_maps0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified UrlMap resource. Gets a list of available URL maps by making a list() request.");
            region_url_maps0 = region_url_maps0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a UrlMap resource in the specified project using the data included in the request.");
            region_url_maps0 = region_url_maps0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves the list of UrlMap resources available to the specified project in the specified region.");
            region_url_maps0 = region_url_maps0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Patches the specified UrlMap resource with the data included in the request. This method supports PATCH semantics and uses JSON merge patch format and processing rules.");
            region_url_maps0 = region_url_maps0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about(
                "Updates the specified UrlMap resource with the data included in the request.",
            );
            region_url_maps0 = region_url_maps0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("validate").about("Runs static validation for the UrlMap. In particular, the tests of the provided UrlMap will be run. Calling this method does NOT create the UrlMap.");
            region_url_maps0 = region_url_maps0.subcommand(mcmd);
        }
        let mut regions0 = SubCommand::with_name("regions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified Region resource. Gets a list of available regions by making a list() request.");
            regions0 = regions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Retrieves the list of region resources available to the specified project.",
            );
            regions0 = regions0.subcommand(mcmd);
        }
        let mut reservations0 = SubCommand::with_name("reservations")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: aggregated_list, delete, get, get_iam_policy, insert, list, resize, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("aggregated_list")
                .about("Retrieves an aggregated list of reservations.");
            reservations0 = reservations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified reservation.");
            reservations0 = reservations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Retrieves information about the specified reservation.");
            reservations0 = reservations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. May be empty if no such policy or resource exists.");
            reservations0 = reservations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about(
                "Creates a new reservation. For more information, read Reserving zonal resources.",
            );
            reservations0 = reservations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("A list of all the reservations that have been configured for the specified project in specified zone.");
            reservations0 = reservations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("resize").about("Resizes the reservation (applicable to standalone reservations only). For more information, read Modifying reservations.");
            reservations0 = reservations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy.");
            reservations0 = reservations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions")
                .about("Returns permissions that a caller has on the specified resource.");
            reservations0 = reservations0.subcommand(mcmd);
        }
        let mut resource_policies0 = SubCommand::with_name("resource_policies")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: aggregated_list, delete, get, get_iam_policy, insert, list, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("aggregated_list")
                .about("Retrieves an aggregated list of resource policies.");
            resource_policies0 = resource_policies0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes the specified resource policy.");
            resource_policies0 = resource_policies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Retrieves all information of the specified resource policy.");
            resource_policies0 = resource_policies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. May be empty if no such policy or resource exists.");
            resource_policies0 = resource_policies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a new resource policy.");
            resource_policies0 = resource_policies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("A list all the resource policies that have been configured for the specified project in specified region.");
            resource_policies0 = resource_policies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy.");
            resource_policies0 = resource_policies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions")
                .about("Returns permissions that a caller has on the specified resource.");
            resource_policies0 = resource_policies0.subcommand(mcmd);
        }
        let mut routers0 = SubCommand::with_name("routers")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: aggregated_list, delete, get, get_nat_mapping_info, get_router_status, insert, list, patch, preview and update");
        {
            let mcmd = SubCommand::with_name("aggregated_list")
                .about("Retrieves an aggregated list of routers.");
            routers0 = routers0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes the specified Router resource.");
            routers0 = routers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified Router resource. Gets a list of available routers by making a list() request.");
            routers0 = routers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_nat_mapping_info")
                .about("Retrieves runtime Nat mapping information of VM endpoints.");
            routers0 = routers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_router_status")
                .about("Retrieves runtime information of the specified router.");
            routers0 = routers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a Router resource in the specified project and region using the data included in the request.");
            routers0 = routers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a list of Router resources available to the specified project.");
            routers0 = routers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Patches the specified Router resource with the data included in the request. This method supports PATCH semantics and uses JSON merge patch format and processing rules.");
            routers0 = routers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("preview").about("Preview fields auto-generated during router create and update operations. Calling this method does NOT create or update the router.");
            routers0 = routers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates the specified Router resource with the data included in the request. This method conforms to PUT semantics, which requests that the state of the target resource be created or replaced with the state defined by the representation enclosed in the request message payload.");
            routers0 = routers0.subcommand(mcmd);
        }
        let mut routes0 = SubCommand::with_name("routes")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert and list");
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes the specified Route resource.");
            routes0 = routes0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified Route resource. Gets a list of available routes by making a list() request.");
            routes0 = routes0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a Route resource in the specified project using the data included in the request.");
            routes0 = routes0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves the list of Route resources available to the specified project.");
            routes0 = routes0.subcommand(mcmd);
        }
        let mut security_policies0 = SubCommand::with_name("security_policies")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: add_rule, delete, get, get_rule, insert, list, list_preconfigured_expression_sets, patch, patch_rule and remove_rule");
        {
            let mcmd =
                SubCommand::with_name("add_rule").about("Inserts a rule into a security policy.");
            security_policies0 = security_policies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified policy.");
            security_policies0 = security_policies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("List all of the ordered rules present in a single specified policy.");
            security_policies0 = security_policies0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get_rule").about("Gets a rule at the specified priority.");
            security_policies0 = security_policies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a new policy in the specified project using the data included in the request.");
            security_policies0 = security_policies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "List all the policies that have been configured for the specified project.",
            );
            security_policies0 = security_policies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list_preconfigured_expression_sets").about("Gets the current list of preconfigured Web Application Firewall (WAF) expressions.");
            security_policies0 = security_policies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Patches the specified policy with the data included in the request.");
            security_policies0 = security_policies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch_rule")
                .about("Patches a rule at the specified priority.");
            security_policies0 = security_policies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("remove_rule")
                .about("Deletes a rule at the specified priority.");
            security_policies0 = security_policies0.subcommand(mcmd);
        }
        let mut snapshots0 = SubCommand::with_name("snapshots")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: delete, get, get_iam_policy, list, set_iam_policy, set_labels and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified Snapshot resource. Keep in mind that deleting a single snapshot might not necessarily delete all the data on that snapshot. If any data on the snapshot that is marked for deletion is needed for subsequent snapshots, the data will be moved to the next corresponding snapshot.\n\nFor more information, see Deleting snapshots.");
            snapshots0 = snapshots0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified Snapshot resource. Gets a list of available snapshots by making a list() request.");
            snapshots0 = snapshots0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. May be empty if no such policy or resource exists.");
            snapshots0 = snapshots0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Retrieves the list of Snapshot resources contained within the specified project.",
            );
            snapshots0 = snapshots0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy.");
            snapshots0 = snapshots0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_labels").about("Sets the labels on a snapshot. To learn more about labels, read the Labeling Resources documentation.");
            snapshots0 = snapshots0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions")
                .about("Returns permissions that a caller has on the specified resource.");
            snapshots0 = snapshots0.subcommand(mcmd);
        }
        let mut ssl_certificates0 = SubCommand::with_name("ssl_certificates")
            .setting(AppSettings::ColoredHelp)
            .about("methods: aggregated_list, delete, get, insert and list");
        {
            let mcmd = SubCommand::with_name("aggregated_list").about("Retrieves the list of all SslCertificate resources, regional and global, available to the specified project.");
            ssl_certificates0 = ssl_certificates0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes the specified SslCertificate resource.");
            ssl_certificates0 = ssl_certificates0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified SslCertificate resource. Gets a list of available SSL certificates by making a list() request.");
            ssl_certificates0 = ssl_certificates0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a SslCertificate resource in the specified project using the data included in the request.");
            ssl_certificates0 = ssl_certificates0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves the list of SslCertificate resources available to the specified project.");
            ssl_certificates0 = ssl_certificates0.subcommand(mcmd);
        }
        let mut ssl_policies0 = SubCommand::with_name("ssl_policies")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, list_available_features and patch");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified SSL policy. The SSL policy resource can be deleted only if it is not in use by any TargetHttpsProxy or TargetSslProxy resources.");
            ssl_policies0 = ssl_policies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Lists all of the ordered rules present in a single specified policy.");
            ssl_policies0 = ssl_policies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Returns the specified SSL policy resource. Gets a list of available SSL policies by making a list() request.");
            ssl_policies0 = ssl_policies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Lists all the SSL policies that have been configured for the specified project.",
            );
            ssl_policies0 = ssl_policies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list_available_features").about("Lists all features that can be specified in the SSL policy when using custom profile.");
            ssl_policies0 = ssl_policies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Patches the specified SSL policy with the data included in the request.");
            ssl_policies0 = ssl_policies0.subcommand(mcmd);
        }
        let mut subnetworks0 = SubCommand::with_name("subnetworks")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: aggregated_list, delete, expand_ip_cidr_range, get, get_iam_policy, insert, list, list_usable, patch, set_iam_policy, set_private_ip_google_access and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("aggregated_list")
                .about("Retrieves an aggregated list of subnetworks.");
            subnetworks0 = subnetworks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified subnetwork.");
            subnetworks0 = subnetworks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("expand_ip_cidr_range")
                .about("Expands the IP CIDR range of the subnetwork to a specified value.");
            subnetworks0 = subnetworks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified subnetwork. Gets a list of available subnetworks list() request.");
            subnetworks0 = subnetworks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. May be empty if no such policy or resource exists.");
            subnetworks0 = subnetworks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a subnetwork in the specified project using the data included in the request.");
            subnetworks0 = subnetworks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a list of subnetworks available to the specified project.");
            subnetworks0 = subnetworks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list_usable").about("Retrieves an aggregated list of all usable subnetworks in the project. The list contains all of the subnetworks in the project and the subnetworks that were shared by a Shared VPC host project.");
            subnetworks0 = subnetworks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Patches the specified subnetwork with the data included in the request. Only certain fields can up updated with a patch request as indicated in the field descriptions. You must specify the current fingerprint of the subnetwork resource being patched.");
            subnetworks0 = subnetworks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy.");
            subnetworks0 = subnetworks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_private_ip_google_access").about("Set whether VMs in this subnet can access Google services without assigning external IP addresses through Private Google Access.");
            subnetworks0 = subnetworks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions")
                .about("Returns permissions that a caller has on the specified resource.");
            subnetworks0 = subnetworks0.subcommand(mcmd);
        }
        let mut target_http_proxies0 = SubCommand::with_name("target_http_proxies")
            .setting(AppSettings::ColoredHelp)
            .about("methods: aggregated_list, delete, get, insert, list and set_url_map");
        {
            let mcmd = SubCommand::with_name("aggregated_list").about("Retrieves the list of all TargetHttpProxy resources, regional and global, available to the specified project.");
            target_http_proxies0 = target_http_proxies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes the specified TargetHttpProxy resource.");
            target_http_proxies0 = target_http_proxies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified TargetHttpProxy resource. Gets a list of available target HTTP proxies by making a list() request.");
            target_http_proxies0 = target_http_proxies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a TargetHttpProxy resource in the specified project using the data included in the request.");
            target_http_proxies0 = target_http_proxies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves the list of TargetHttpProxy resources available to the specified project.");
            target_http_proxies0 = target_http_proxies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_url_map")
                .about("Changes the URL map for TargetHttpProxy.");
            target_http_proxies0 = target_http_proxies0.subcommand(mcmd);
        }
        let mut target_https_proxies0 = SubCommand::with_name("target_https_proxies")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: aggregated_list, delete, get, insert, list, set_quic_override, set_ssl_certificates, set_ssl_policy and set_url_map");
        {
            let mcmd = SubCommand::with_name("aggregated_list").about("Retrieves the list of all TargetHttpsProxy resources, regional and global, available to the specified project.");
            target_https_proxies0 = target_https_proxies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes the specified TargetHttpsProxy resource.");
            target_https_proxies0 = target_https_proxies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified TargetHttpsProxy resource. Gets a list of available target HTTPS proxies by making a list() request.");
            target_https_proxies0 = target_https_proxies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a TargetHttpsProxy resource in the specified project using the data included in the request.");
            target_https_proxies0 = target_https_proxies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves the list of TargetHttpsProxy resources available to the specified project.");
            target_https_proxies0 = target_https_proxies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_quic_override")
                .about("Sets the QUIC override policy for TargetHttpsProxy.");
            target_https_proxies0 = target_https_proxies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_ssl_certificates")
                .about("Replaces SslCertificates for TargetHttpsProxy.");
            target_https_proxies0 = target_https_proxies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_ssl_policy").about("Sets the SSL policy for TargetHttpsProxy. The SSL policy specifies the server-side support for SSL features. This affects connections between clients and the HTTPS proxy load balancer. They do not affect the connection between the load balancer and the backends.");
            target_https_proxies0 = target_https_proxies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_url_map")
                .about("Changes the URL map for TargetHttpsProxy.");
            target_https_proxies0 = target_https_proxies0.subcommand(mcmd);
        }
        let mut target_instances0 = SubCommand::with_name("target_instances")
            .setting(AppSettings::ColoredHelp)
            .about("methods: aggregated_list, delete, get, insert and list");
        {
            let mcmd = SubCommand::with_name("aggregated_list")
                .about("Retrieves an aggregated list of target instances.");
            target_instances0 = target_instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes the specified TargetInstance resource.");
            target_instances0 = target_instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified TargetInstance resource. Gets a list of available target instances by making a list() request.");
            target_instances0 = target_instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a TargetInstance resource in the specified project and zone using the data included in the request.");
            target_instances0 = target_instances0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves a list of TargetInstance resources available to the specified project and zone.");
            target_instances0 = target_instances0.subcommand(mcmd);
        }
        let mut target_pools0 = SubCommand::with_name("target_pools")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: add_health_check, add_instance, aggregated_list, delete, get, get_health, insert, list, remove_health_check, remove_instance and set_backup");
        {
            let mcmd = SubCommand::with_name("add_health_check")
                .about("Adds health check URLs to a target pool.");
            target_pools0 = target_pools0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("add_instance").about("Adds an instance to a target pool.");
            target_pools0 = target_pools0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("aggregated_list")
                .about("Retrieves an aggregated list of target pools.");
            target_pools0 = target_pools0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified target pool.");
            target_pools0 = target_pools0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified target pool. Gets a list of available target pools by making a list() request.");
            target_pools0 = target_pools0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_health").about("Gets the most recent health check results for each IP for the instance that is referenced by the given target pool.");
            target_pools0 = target_pools0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a target pool in the specified project and region using the data included in the request.");
            target_pools0 = target_pools0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Retrieves a list of target pools available to the specified project and region.",
            );
            target_pools0 = target_pools0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("remove_health_check")
                .about("Removes health check URL from a target pool.");
            target_pools0 = target_pools0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("remove_instance")
                .about("Removes instance URL from a target pool.");
            target_pools0 = target_pools0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_backup")
                .about("Changes a backup target pool\'s configurations.");
            target_pools0 = target_pools0.subcommand(mcmd);
        }
        let mut target_ssl_proxies0 = SubCommand::with_name("target_ssl_proxies")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: delete, get, insert, list, set_backend_service, set_proxy_header, set_ssl_certificates and set_ssl_policy");
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes the specified TargetSslProxy resource.");
            target_ssl_proxies0 = target_ssl_proxies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified TargetSslProxy resource. Gets a list of available target SSL proxies by making a list() request.");
            target_ssl_proxies0 = target_ssl_proxies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a TargetSslProxy resource in the specified project using the data included in the request.");
            target_ssl_proxies0 = target_ssl_proxies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves the list of TargetSslProxy resources available to the specified project.");
            target_ssl_proxies0 = target_ssl_proxies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_backend_service")
                .about("Changes the BackendService for TargetSslProxy.");
            target_ssl_proxies0 = target_ssl_proxies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_proxy_header")
                .about("Changes the ProxyHeaderType for TargetSslProxy.");
            target_ssl_proxies0 = target_ssl_proxies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_ssl_certificates")
                .about("Changes SslCertificates for TargetSslProxy.");
            target_ssl_proxies0 = target_ssl_proxies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_ssl_policy").about("Sets the SSL policy for TargetSslProxy. The SSL policy specifies the server-side support for SSL features. This affects connections between clients and the SSL proxy load balancer. They do not affect the connection between the load balancer and the backends.");
            target_ssl_proxies0 = target_ssl_proxies0.subcommand(mcmd);
        }
        let mut target_tcp_proxies0 = SubCommand::with_name("target_tcp_proxies")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, set_backend_service and set_proxy_header");
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes the specified TargetTcpProxy resource.");
            target_tcp_proxies0 = target_tcp_proxies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified TargetTcpProxy resource. Gets a list of available target TCP proxies by making a list() request.");
            target_tcp_proxies0 = target_tcp_proxies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a TargetTcpProxy resource in the specified project using the data included in the request.");
            target_tcp_proxies0 = target_tcp_proxies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves the list of TargetTcpProxy resources available to the specified project.");
            target_tcp_proxies0 = target_tcp_proxies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_backend_service")
                .about("Changes the BackendService for TargetTcpProxy.");
            target_tcp_proxies0 = target_tcp_proxies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_proxy_header")
                .about("Changes the ProxyHeaderType for TargetTcpProxy.");
            target_tcp_proxies0 = target_tcp_proxies0.subcommand(mcmd);
        }
        let mut target_vpn_gateways0 = SubCommand::with_name("target_vpn_gateways")
            .setting(AppSettings::ColoredHelp)
            .about("methods: aggregated_list, delete, get, insert and list");
        {
            let mcmd = SubCommand::with_name("aggregated_list")
                .about("Retrieves an aggregated list of target VPN gateways.");
            target_vpn_gateways0 = target_vpn_gateways0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes the specified target VPN gateway.");
            target_vpn_gateways0 = target_vpn_gateways0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified target VPN gateway. Gets a list of available target VPN gateways by making a list() request.");
            target_vpn_gateways0 = target_vpn_gateways0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a target VPN gateway in the specified project and region using the data included in the request.");
            target_vpn_gateways0 = target_vpn_gateways0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves a list of target VPN gateways available to the specified project and region.");
            target_vpn_gateways0 = target_vpn_gateways0.subcommand(mcmd);
        }
        let mut url_maps0 = SubCommand::with_name("url_maps")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: aggregated_list, delete, get, insert, invalidate_cache, list, patch, update and validate");
        {
            let mcmd = SubCommand::with_name("aggregated_list").about("Retrieves the list of all UrlMap resources, regional and global, available to the specified project.");
            url_maps0 = url_maps0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes the specified UrlMap resource.");
            url_maps0 = url_maps0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified UrlMap resource. Gets a list of available URL maps by making a list() request.");
            url_maps0 = url_maps0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a UrlMap resource in the specified project using the data included in the request.");
            url_maps0 = url_maps0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("invalidate_cache").about("Initiates a cache invalidation operation, invalidating the specified path, scoped to the specified UrlMap.");
            url_maps0 = url_maps0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Retrieves the list of UrlMap resources available to the specified project.",
            );
            url_maps0 = url_maps0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Patches the specified UrlMap resource with the data included in the request. This method supports PATCH semantics and uses the JSON merge patch format and processing rules.");
            url_maps0 = url_maps0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about(
                "Updates the specified UrlMap resource with the data included in the request.",
            );
            url_maps0 = url_maps0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("validate").about("Runs static validation for the UrlMap. In particular, the tests of the provided UrlMap will be run. Calling this method does NOT create the UrlMap.");
            url_maps0 = url_maps0.subcommand(mcmd);
        }
        let mut vpn_gateways0 = SubCommand::with_name("vpn_gateways")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: aggregated_list, delete, get, get_status, insert, list, set_labels and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("aggregated_list")
                .about("Retrieves an aggregated list of VPN gateways.");
            vpn_gateways0 = vpn_gateways0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified VPN gateway.");
            vpn_gateways0 = vpn_gateways0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified VPN gateway. Gets a list of available VPN gateways by making a list() request.");
            vpn_gateways0 = vpn_gateways0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_status")
                .about("Returns the status for the specified VPN gateway.");
            vpn_gateways0 = vpn_gateways0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a VPN gateway in the specified project and region using the data included in the request.");
            vpn_gateways0 = vpn_gateways0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Retrieves a list of VPN gateways available to the specified project and region.",
            );
            vpn_gateways0 = vpn_gateways0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_labels").about("Sets the labels on a VpnGateway. To learn more about labels, read the Labeling Resources documentation.");
            vpn_gateways0 = vpn_gateways0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions")
                .about("Returns permissions that a caller has on the specified resource.");
            vpn_gateways0 = vpn_gateways0.subcommand(mcmd);
        }
        let mut vpn_tunnels0 = SubCommand::with_name("vpn_tunnels")
            .setting(AppSettings::ColoredHelp)
            .about("methods: aggregated_list, delete, get, insert and list");
        {
            let mcmd = SubCommand::with_name("aggregated_list")
                .about("Retrieves an aggregated list of VPN tunnels.");
            vpn_tunnels0 = vpn_tunnels0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes the specified VpnTunnel resource.");
            vpn_tunnels0 = vpn_tunnels0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified VpnTunnel resource. Gets a list of available VPN tunnels by making a list() request.");
            vpn_tunnels0 = vpn_tunnels0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a VpnTunnel resource in the specified project and region using the data included in the request.");
            vpn_tunnels0 = vpn_tunnels0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves a list of VpnTunnel resources contained in the specified project and region.");
            vpn_tunnels0 = vpn_tunnels0.subcommand(mcmd);
        }
        let mut zone_operations0 = SubCommand::with_name("zone_operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, list and wait");
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes the specified zone-specific Operations resource.");
            zone_operations0 = zone_operations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Retrieves the specified zone-specific Operations resource.");
            zone_operations0 = zone_operations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Retrieves a list of Operation resources contained within the specified zone.",
            );
            zone_operations0 = zone_operations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("wait").about("Waits for the specified Operation resource to return as `DONE` or for the request to approach the 2 minute deadline, and retrieves the specified Operation resource. This method differs from the `GET` method in that it waits for no more than the default deadline (2 minutes) and then returns the current state of the operation, which might be `DONE` or still in progress.\n\nThis method is called on a best-effort basis. Specifically:  \n- In uncommon cases, when the server is overloaded, the request might return before the default deadline is reached, or might return after zero seconds. \n- If the default deadline is reached, there is no guarantee that the operation is actually done when the method returns. Be prepared to retry if the operation is not `DONE`.");
            zone_operations0 = zone_operations0.subcommand(mcmd);
        }
        let mut zones0 = SubCommand::with_name("zones")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified Zone resource. Gets a list of available zones by making a list() request.");
            zones0 = zones0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves the list of Zone resources available to the specified project.");
            zones0 = zones0.subcommand(mcmd);
        }
        app = app.subcommand(zones0);
        app = app.subcommand(zone_operations0);
        app = app.subcommand(vpn_tunnels0);
        app = app.subcommand(vpn_gateways0);
        app = app.subcommand(url_maps0);
        app = app.subcommand(target_vpn_gateways0);
        app = app.subcommand(target_tcp_proxies0);
        app = app.subcommand(target_ssl_proxies0);
        app = app.subcommand(target_pools0);
        app = app.subcommand(target_instances0);
        app = app.subcommand(target_https_proxies0);
        app = app.subcommand(target_http_proxies0);
        app = app.subcommand(subnetworks0);
        app = app.subcommand(ssl_policies0);
        app = app.subcommand(ssl_certificates0);
        app = app.subcommand(snapshots0);
        app = app.subcommand(security_policies0);
        app = app.subcommand(routes0);
        app = app.subcommand(routers0);
        app = app.subcommand(resource_policies0);
        app = app.subcommand(reservations0);
        app = app.subcommand(regions0);
        app = app.subcommand(region_url_maps0);
        app = app.subcommand(region_target_https_proxies0);
        app = app.subcommand(region_target_http_proxies0);
        app = app.subcommand(region_ssl_certificates0);
        app = app.subcommand(region_operations0);
        app = app.subcommand(region_notification_endpoints0);
        app = app.subcommand(region_instance_groups0);
        app = app.subcommand(region_instance_group_managers0);
        app = app.subcommand(region_health_checks0);
        app = app.subcommand(region_health_check_services0);
        app = app.subcommand(region_disks0);
        app = app.subcommand(region_disk_types0);
        app = app.subcommand(region_commitments0);
        app = app.subcommand(region_backend_services0);
        app = app.subcommand(region_autoscalers0);
        app = app.subcommand(projects0);
        app = app.subcommand(packet_mirrorings0);
        app = app.subcommand(node_types0);
        app = app.subcommand(node_templates0);
        app = app.subcommand(node_groups0);
        app = app.subcommand(networks0);
        app = app.subcommand(network_endpoint_groups0);
        app = app.subcommand(machine_types0);
        app = app.subcommand(licenses0);
        app = app.subcommand(license_codes0);
        app = app.subcommand(interconnects0);
        app = app.subcommand(interconnect_locations0);
        app = app.subcommand(interconnect_attachments0);
        app = app.subcommand(instances0);
        app = app.subcommand(instance_templates0);
        app = app.subcommand(instance_groups0);
        app = app.subcommand(instance_group_managers0);
        app = app.subcommand(images0);
        app = app.subcommand(https_health_checks0);
        app = app.subcommand(http_health_checks0);
        app = app.subcommand(health_checks0);
        app = app.subcommand(global_operations0);
        app = app.subcommand(global_network_endpoint_groups0);
        app = app.subcommand(global_forwarding_rules0);
        app = app.subcommand(global_addresses0);
        app = app.subcommand(forwarding_rules0);
        app = app.subcommand(firewalls0);
        app = app.subcommand(external_vpn_gateways0);
        app = app.subcommand(disks0);
        app = app.subcommand(disk_types0);
        app = app.subcommand(backend_services0);
        app = app.subcommand(backend_buckets0);
        app = app.subcommand(autoscalers0);
        app = app.subcommand(addresses0);
        app = app.subcommand(accelerator_types0);

        Self { app }
    }
}
use google_compute1 as api;

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
