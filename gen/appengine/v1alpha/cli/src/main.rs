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
        let mut app = App::new("appengine1_alpha")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200608")
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
                        .about("sub-resources: authorized_certificates, authorized_domains, domain_mappings, locations and operations");
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
        apps0 = apps0.subcommand(operations1);
        apps0 = apps0.subcommand(locations1);
        apps0 = apps0.subcommand(domain_mappings1);
        apps0 = apps0.subcommand(authorized_domains1);
        apps0 = apps0.subcommand(authorized_certificates1);
        app = app.subcommand(apps0);

        Self { app }
    }
}
use google_appengine1_alpha as api;

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
