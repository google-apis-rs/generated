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
        let mut app = App::new("appengine1_beta")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200410")
            .about("Provisions and manages developers\' App Engine applications.")
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
        let mut apps0 = SubCommand::with_name("apps")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, get, patch and repair");
        {
            let mcmd = SubCommand::with_name("create").about("Creates an App Engine application for a Google Cloud Platform project. Required fields:\nid - The ID of the target Cloud Platform project.\nlocation - The region (https://cloud.google.com/appengine/docs/locations) where you want the App Engine application located.For more information about App Engine applications, see Managing Projects, Applications, and Billing (https://cloud.google.com/appengine/docs/standard/python/console/).");
            apps0 = apps0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets information about an application.");
            apps0 = apps0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified Application resource. You can update the following fields:\nauth_domain - Google authentication domain for controlling user access to the application.\ndefault_cookie_expiration - Cookie expiration policy for the application.");
            apps0 = apps0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("repair").about("Recreates the required App Engine features for the specified App Engine application, for example a Cloud Storage bucket or App Engine service account. Use this method if you receive an error message about a missing feature, for example, Error retrieving the App Engine service account. If you have deleted your App Engine service account, this will not be able to recreate it. Instead, you should attempt to use the IAM undelete API if possible at https://cloud.google.com/iam/reference/rest/v1/projects.serviceAccounts/undelete?apix_params=%7B\"name\"%3A\"projects%2F-%2FserviceAccounts%2Funique_id\"%2C\"resource\"%3A%7B%7D%7D . If the deletion was recent, the numeric ID can be found in the Cloud Console Activity Log.");
            apps0 = apps0.subcommand(mcmd);
        }
        let mut authorized_certificates1 = SubCommand::with_name("authorized_certificates")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd =
                SubCommand::with_name("create").about("Uploads the specified SSL certificate.");
            authorized_certificates1 = authorized_certificates1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes the specified SSL certificate.");
            authorized_certificates1 = authorized_certificates1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the specified SSL certificate.");
            authorized_certificates1 = authorized_certificates1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all SSL certificates the user is authorized to administer.");
            authorized_certificates1 = authorized_certificates1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified SSL certificate. To renew a certificate and maintain its existing domain mappings, update certificate_data with a new certificate. The new certificate must be applicable to the same domains as the original certificate. The certificate display_name may also be updated.");
            authorized_certificates1 = authorized_certificates1.subcommand(mcmd);
        }
        let mut authorized_domains1 = SubCommand::with_name("authorized_domains")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all domains the user is authorized to administer.");
            authorized_domains1 = authorized_domains1.subcommand(mcmd);
        }
        let mut domain_mappings1 = SubCommand::with_name("domain_mappings")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Maps a domain to an application. A user must be authorized to administer a domain in order to map it to an application. For a list of available authorized domains, see AuthorizedDomains.ListAuthorizedDomains.");
            domain_mappings1 = domain_mappings1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified domain mapping. A user must be authorized to administer the associated domain in order to delete a DomainMapping resource.");
            domain_mappings1 = domain_mappings1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the specified domain mapping.");
            domain_mappings1 = domain_mappings1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists the domain mappings on an application.");
            domain_mappings1 = domain_mappings1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified domain mapping. To map an SSL certificate to a domain mapping, update certificate_id to point to an AuthorizedCertificate resource. A user must be authorized to administer the associated domain in order to update a DomainMapping resource.");
            domain_mappings1 = domain_mappings1.subcommand(mcmd);
        }
        let mut firewall1 = SubCommand::with_name("firewall")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: ingress_rules");
        let mut locations1 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets information about a location.");
            locations1 = locations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists information about the supported locations for this service.");
            locations1 = locations1.subcommand(mcmd);
        }
        let mut operations1 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations1 = operations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request. If the server doesn\'t support this method, it returns UNIMPLEMENTED.NOTE: the name binding allows API services to override the binding to use different resource name schemes, such as users/*/operations. To override the binding, API services can add a binding such as \"/v1/{name=users/*}/operations\" to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.");
            operations1 = operations1.subcommand(mcmd);
        }
        let mut services1 = SubCommand::with_name("services")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes the specified service and all enclosed versions.");
            services1 = services1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets the current configuration of the specified service.");
            services1 = services1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists all the services in the application.");
            services1 = services1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates the configuration of the specified service.");
            services1 = services1.subcommand(mcmd);
        }
        let mut ingress_rules2 = SubCommand::with_name("ingress_rules")
            .setting(AppSettings::ColoredHelp)
            .about("methods: batch_update, create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("batch_update").about("Replaces the entire firewall ruleset in one bulk operation. This overrides and replaces the rules of an existing firewall with the new rules.If the final rule does not match traffic with the \'*\' wildcard IP range, then an \"allow all\" rule is explicitly added to the end of the list.");
            ingress_rules2 = ingress_rules2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a firewall rule for the application.");
            ingress_rules2 = ingress_rules2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes the specified firewall rule.");
            ingress_rules2 = ingress_rules2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the specified firewall rule.");
            ingress_rules2 = ingress_rules2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists the firewall rules of an application.");
            ingress_rules2 = ingress_rules2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified firewall rule.");
            ingress_rules2 = ingress_rules2.subcommand(mcmd);
        }
        let mut versions2 = SubCommand::with_name("versions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Deploys code and resource files to a new version.");
            versions2 = versions2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes an existing Version resource.");
            versions2 = versions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the specified Version resource. By default, only a BASIC_VIEW will be returned. Specify the FULL_VIEW parameter to get the full resource.");
            versions2 = versions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the versions of a service.");
            versions2 = versions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified Version resource. You can specify the following fields depending on the App Engine environment and type of scaling that the version resource uses:Standard environment\ninstance_class (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta/apps.services.versions#Version.FIELDS.instance_class)automatic scaling in the standard environment:\nautomatic_scaling.min_idle_instances (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta/apps.services.versions#Version.FIELDS.automatic_scaling)\nautomatic_scaling.max_idle_instances (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta/apps.services.versions#Version.FIELDS.automatic_scaling)\nautomaticScaling.standard_scheduler_settings.max_instances (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta/apps.services.versions#StandardSchedulerSettings)\nautomaticScaling.standard_scheduler_settings.min_instances (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta/apps.services.versions#StandardSchedulerSettings)\nautomaticScaling.standard_scheduler_settings.target_cpu_utilization (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta/apps.services.versions#StandardSchedulerSettings)\nautomaticScaling.standard_scheduler_settings.target_throughput_utilization (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta/apps.services.versions#StandardSchedulerSettings)basic scaling or manual scaling in the standard environment:\nserving_status (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta/apps.services.versions#Version.FIELDS.serving_status)Flexible environment\nserving_status (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta/apps.services.versions#Version.FIELDS.serving_status)automatic scaling in the flexible environment:\nautomatic_scaling.min_total_instances (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta/apps.services.versions#Version.FIELDS.automatic_scaling)\nautomatic_scaling.max_total_instances (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta/apps.services.versions#Version.FIELDS.automatic_scaling)\nautomatic_scaling.cool_down_period_sec (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta/apps.services.versions#Version.FIELDS.automatic_scaling)\nautomatic_scaling.cpu_utilization.target_utilization (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta/apps.services.versions#Version.FIELDS.automatic_scaling)");
            versions2 = versions2.subcommand(mcmd);
        }
        let mut instances3 = SubCommand::with_name("instances")
            .setting(AppSettings::ColoredHelp)
            .about("methods: debug, delete, get and list");
        {
            let mcmd = SubCommand::with_name("debug").about("Enables debugging on a VM instance. This allows you to use the SSH command to connect to the virtual machine where the instance lives. While in \"debug mode\", the instance continues to serve live traffic. You should delete the instance when you are done debugging and then allow the system to take over and determine if another instance should be started.Only applicable for instances in App Engine flexible environment.");
            instances3 = instances3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Stops a running instance.The instance might be automatically recreated based on the scaling settings of the version. For more information, see \"How Instances are Managed\" (standard environment (https://cloud.google.com/appengine/docs/standard/python/how-instances-are-managed) | flexible environment (https://cloud.google.com/appengine/docs/flexible/python/how-instances-are-managed)).To ensure that instances are not re-created and avoid getting billed, you can stop all instances within the target version by changing the serving status of the version to STOPPED with the apps.services.versions.patch (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions/patch) method.");
            instances3 = instances3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets instance information.");
            instances3 = instances3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the instances of a version.Tip: To aggregate details about instances over time, see the Stackdriver Monitoring API (https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.timeSeries/list).");
            instances3 = instances3.subcommand(mcmd);
        }
        versions2 = versions2.subcommand(instances3);
        services1 = services1.subcommand(versions2);
        firewall1 = firewall1.subcommand(ingress_rules2);
        apps0 = apps0.subcommand(services1);
        apps0 = apps0.subcommand(operations1);
        apps0 = apps0.subcommand(locations1);
        apps0 = apps0.subcommand(firewall1);
        apps0 = apps0.subcommand(domain_mappings1);
        apps0 = apps0.subcommand(authorized_domains1);
        apps0 = apps0.subcommand(authorized_certificates1);
        app = app.subcommand(apps0);

        Self { app }
    }
}
use google_appengine1_beta as api;

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
