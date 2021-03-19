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
        let mut app = App::new("container1_beta1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20210311")
            .about("Builds and manages container-based applications, powered by the open source Kubernetes technology.")
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
            .about("sub-resources: aggregated, locations and zones");
        let mut aggregated1 = SubCommand::with_name("aggregated")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: usable_subnetworks");
        let mut locations1 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get_server_config and list");
        {
            let mcmd = SubCommand::with_name("get_server_config")
                .about("Returns configuration info about the Google Kubernetes Engine service.");
            locations1 = locations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Fetches locations that offer Google Kubernetes Engine.");
            locations1 = locations1.subcommand(mcmd);
        }
        let mut zones1 = SubCommand::with_name("zones")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get_serverconfig");
        {
            let mcmd = SubCommand::with_name("get_serverconfig")
                .about("Returns configuration info about the Google Kubernetes Engine service.");
            zones1 = zones1.subcommand(mcmd);
        }
        let mut usable_subnetworks2 = SubCommand::with_name("usable_subnetworks")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists subnetworks that can be used for creating clusters in a project.");
            usable_subnetworks2 = usable_subnetworks2.subcommand(mcmd);
        }
        let mut clusters2 = SubCommand::with_name("clusters")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: complete_ip_rotation, create, delete, get, get_jwks, list, set_addons, set_legacy_abac, set_locations, set_logging, set_maintenance_policy, set_master_auth, set_monitoring, set_network_policy, set_resource_labels, start_ip_rotation, update and update_master");
        {
            let mcmd = SubCommand::with_name("complete_ip_rotation")
                .about("Completes master IP rotation.");
            clusters2 = clusters2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a cluster, consisting of the specified number and type of Google Compute Engine instances. By default, the cluster is created in the project\'s [default network](https://cloud.google.com/compute/docs/networks-and-firewalls#networks). One firewall is added for the cluster. After cluster creation, the Kubelet creates routes for each node to allow the containers on that node to communicate with all other instances in the cluster. Finally, an entry is added to the project\'s global metadata indicating which CIDR range the cluster is using.");
            clusters2 = clusters2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the cluster, including the Kubernetes endpoint and all worker nodes. Firewalls and routes that were configured during cluster creation are also deleted. Other Google Compute Engine resources that might be in use by the cluster, such as load balancer resources, are not deleted if they weren\'t present when the cluster was initially created.");
            clusters2 = clusters2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets the details for a specific cluster.");
            clusters2 = clusters2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_jwks").about("Gets the public component of the cluster signing keys in JSON Web Key format. This API is not yet intended for general use, and is not available for all clusters.");
            clusters2 = clusters2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Lists all clusters owned by a project in either the specified zone or all zones.",
            );
            clusters2 = clusters2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_addons")
                .about("Sets the addons for a specific cluster.");
            clusters2 = clusters2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_legacy_abac")
                .about("Enables or disables the ABAC authorization mechanism on a cluster.");
            clusters2 = clusters2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_locations").about("Sets the locations for a specific cluster. Deprecated. Use [projects.locations.clusters.update](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1beta1/projects.locations.clusters/update) instead.");
            clusters2 = clusters2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_logging")
                .about("Sets the logging service for a specific cluster.");
            clusters2 = clusters2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_maintenance_policy")
                .about("Sets the maintenance policy for a cluster.");
            clusters2 = clusters2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_master_auth").about("Sets master auth materials. Currently supports changing the admin password or a specific cluster, either via password generation or explicitly setting the password.");
            clusters2 = clusters2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_monitoring")
                .about("Sets the monitoring service for a specific cluster.");
            clusters2 = clusters2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_network_policy")
                .about("Enables or disables Network Policy for a cluster.");
            clusters2 = clusters2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("set_resource_labels").about("Sets labels on a cluster.");
            clusters2 = clusters2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("start_ip_rotation").about("Starts master IP rotation.");
            clusters2 = clusters2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update")
                .about("Updates the settings for a specific cluster.");
            clusters2 = clusters2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_master")
                .about("Updates the master for a specific cluster.");
            clusters2 = clusters2.subcommand(mcmd);
        }
        let mut operations2 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, get and list");
        {
            let mcmd = SubCommand::with_name("cancel").about("Cancels the specified operation.");
            operations2 = operations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the specified operation.");
            operations2 = operations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all operations in a project in the specified zone or all zones.");
            operations2 = operations2.subcommand(mcmd);
        }
        let mut clusters2 = SubCommand::with_name("clusters")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: addons, complete_ip_rotation, create, delete, get, legacy_abac, list, locations, logging, master, monitoring, resource_labels, set_maintenance_policy, set_master_auth, set_network_policy, start_ip_rotation and update");
        {
            let mcmd =
                SubCommand::with_name("addons").about("Sets the addons for a specific cluster.");
            clusters2 = clusters2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("complete_ip_rotation")
                .about("Completes master IP rotation.");
            clusters2 = clusters2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a cluster, consisting of the specified number and type of Google Compute Engine instances. By default, the cluster is created in the project\'s [default network](https://cloud.google.com/compute/docs/networks-and-firewalls#networks). One firewall is added for the cluster. After cluster creation, the Kubelet creates routes for each node to allow the containers on that node to communicate with all other instances in the cluster. Finally, an entry is added to the project\'s global metadata indicating which CIDR range the cluster is using.");
            clusters2 = clusters2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the cluster, including the Kubernetes endpoint and all worker nodes. Firewalls and routes that were configured during cluster creation are also deleted. Other Google Compute Engine resources that might be in use by the cluster, such as load balancer resources, are not deleted if they weren\'t present when the cluster was initially created.");
            clusters2 = clusters2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets the details for a specific cluster.");
            clusters2 = clusters2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("legacy_abac")
                .about("Enables or disables the ABAC authorization mechanism on a cluster.");
            clusters2 = clusters2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Lists all clusters owned by a project in either the specified zone or all zones.",
            );
            clusters2 = clusters2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("locations").about("Sets the locations for a specific cluster. Deprecated. Use [projects.locations.clusters.update](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1beta1/projects.locations.clusters/update) instead.");
            clusters2 = clusters2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("logging")
                .about("Sets the logging service for a specific cluster.");
            clusters2 = clusters2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("master").about("Updates the master for a specific cluster.");
            clusters2 = clusters2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("monitoring")
                .about("Sets the monitoring service for a specific cluster.");
            clusters2 = clusters2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("resource_labels").about("Sets labels on a cluster.");
            clusters2 = clusters2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_maintenance_policy")
                .about("Sets the maintenance policy for a cluster.");
            clusters2 = clusters2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_master_auth").about("Sets master auth materials. Currently supports changing the admin password or a specific cluster, either via password generation or explicitly setting the password.");
            clusters2 = clusters2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_network_policy")
                .about("Enables or disables Network Policy for a cluster.");
            clusters2 = clusters2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("start_ip_rotation").about("Starts master IP rotation.");
            clusters2 = clusters2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update")
                .about("Updates the settings for a specific cluster.");
            clusters2 = clusters2.subcommand(mcmd);
        }
        let mut operations2 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, get and list");
        {
            let mcmd = SubCommand::with_name("cancel").about("Cancels the specified operation.");
            operations2 = operations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the specified operation.");
            operations2 = operations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all operations in a project in the specified zone or all zones.");
            operations2 = operations2.subcommand(mcmd);
        }
        let mut node_pools3 = SubCommand::with_name("node_pools")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, list, rollback, set_autoscaling, set_management, set_size and update");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a node pool for a cluster.");
            node_pools3 = node_pools3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a node pool from a cluster.");
            node_pools3 = node_pools3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves the requested node pool.");
            node_pools3 = node_pools3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the node pools for a cluster.");
            node_pools3 = node_pools3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("rollback").about("Rolls back a previously Aborted or Failed NodePool upgrade. This makes no changes if the last upgrade successfully completed.");
            node_pools3 = node_pools3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_autoscaling")
                .about("Sets the autoscaling settings of a specific node pool.");
            node_pools3 = node_pools3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_management")
                .about("Sets the NodeManagement options for a node pool.");
            node_pools3 = node_pools3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_size").about("SetNodePoolSizeRequest sets the size of a node pool. The new size will be used for all replicas, including future replicas created by modifying NodePool.locations.");
            node_pools3 = node_pools3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update")
                .about("Updates the version and/or image type of a specific node pool.");
            node_pools3 = node_pools3.subcommand(mcmd);
        }
        let mut well_known3 = SubCommand::with_name("well_known")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get_openid_configuration");
        {
            let mcmd = SubCommand::with_name("get_openid_configuration").about("Gets the OIDC discovery document for the cluster. See the [OpenID Connect Discovery 1.0 specification](https://openid.net/specs/openid-connect-discovery-1_0.html) for details. This API is not yet intended for general use, and is not available for all clusters.");
            well_known3 = well_known3.subcommand(mcmd);
        }
        let mut node_pools3 = SubCommand::with_name("node_pools")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: autoscaling, create, delete, get, list, rollback, set_management, set_size and update");
        {
            let mcmd = SubCommand::with_name("autoscaling")
                .about("Sets the autoscaling settings of a specific node pool.");
            node_pools3 = node_pools3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a node pool for a cluster.");
            node_pools3 = node_pools3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a node pool from a cluster.");
            node_pools3 = node_pools3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves the requested node pool.");
            node_pools3 = node_pools3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the node pools for a cluster.");
            node_pools3 = node_pools3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("rollback").about("Rolls back a previously Aborted or Failed NodePool upgrade. This makes no changes if the last upgrade successfully completed.");
            node_pools3 = node_pools3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_management")
                .about("Sets the NodeManagement options for a node pool.");
            node_pools3 = node_pools3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_size").about("SetNodePoolSizeRequest sets the size of a node pool. The new size will be used for all replicas, including future replicas created by modifying NodePool.locations.");
            node_pools3 = node_pools3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update")
                .about("Updates the version and/or image type of a specific node pool.");
            node_pools3 = node_pools3.subcommand(mcmd);
        }
        clusters2 = clusters2.subcommand(node_pools3);
        clusters2 = clusters2.subcommand(well_known3);
        clusters2 = clusters2.subcommand(node_pools3);
        zones1 = zones1.subcommand(operations2);
        zones1 = zones1.subcommand(clusters2);
        locations1 = locations1.subcommand(operations2);
        locations1 = locations1.subcommand(clusters2);
        aggregated1 = aggregated1.subcommand(usable_subnetworks2);
        projects0 = projects0.subcommand(zones1);
        projects0 = projects0.subcommand(locations1);
        projects0 = projects0.subcommand(aggregated1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_container1_beta1 as api;

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
