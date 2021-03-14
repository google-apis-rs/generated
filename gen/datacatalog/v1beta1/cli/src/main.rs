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
        let mut app = App::new("datacatalog1_beta1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20210305")
            .about("A fully managed and highly scalable data discovery and metadata management service. ")
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
        let mut catalog0 = SubCommand::with_name("catalog")
            .setting(AppSettings::ColoredHelp)
            .about("methods: search");
        {
            let mcmd = SubCommand::with_name("search").about("Searches Data Catalog for multiple resources like entries, tags that match a query. This is a custom method (https://cloud.google.com/apis/design/custom_methods) and does not return the complete resource, only the resource identifier and high level fields. Clients can subsequently call `Get` methods. Note that Data Catalog search queries do not guarantee full recall. Query results that match your query may not be returned, even in subsequent result pages. Also note that results returned (and not returned) can vary across repeated search queries. See [Data Catalog Search Syntax](https://cloud.google.com/data-catalog/docs/how-to/search-reference) for more information.");
            catalog0 = catalog0.subcommand(mcmd);
        }
        let mut entries0 = SubCommand::with_name("entries")
            .setting(AppSettings::ColoredHelp)
            .about("methods: lookup");
        {
            let mcmd = SubCommand::with_name("lookup").about("Get an entry by target resource name. This method allows clients to use the resource name from the source Google Cloud Platform service to get the Data Catalog Entry.");
            entries0 = entries0.subcommand(mcmd);
        }
        let mut projects0 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: locations");
        let mut locations1 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: entry_groups, tag_templates and taxonomies");
        let mut entry_groups2 = SubCommand::with_name("entry_groups")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_iam_policy, list, patch, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create").about("A maximum of 10,000 entry groups may be created per organization across all locations. Users should enable the Data Catalog API in the project identified by the `parent` parameter (see [Data Catalog Resource Project] (https://cloud.google.com/data-catalog/docs/concepts/resource-project) for more information).");
            entry_groups2 = entry_groups2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an EntryGroup. Only entry groups that do not contain entries can be deleted. Users should enable the Data Catalog API in the project identified by the `name` parameter (see [Data Catalog Resource Project] (https://cloud.google.com/data-catalog/docs/concepts/resource-project) for more information).");
            entry_groups2 = entry_groups2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets an EntryGroup.");
            entry_groups2 = entry_groups2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. A `NOT_FOUND` error is returned if the resource does not exist. An empty policy is returned if the resource exists but does not have a policy set on it. Supported resources are: - Tag templates. - Entries. - Entry groups. Note, this method cannot be used to manage policies for BigQuery, Pub/Sub and any external Google Cloud Platform resources synced to Data Catalog. Callers must have following Google IAM permission - `datacatalog.tagTemplates.getIamPolicy` to get policies on tag templates. - `datacatalog.entries.getIamPolicy` to get policies on entries. - `datacatalog.entryGroups.getIamPolicy` to get policies on entry groups.");
            entry_groups2 = entry_groups2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists entry groups.");
            entry_groups2 = entry_groups2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an EntryGroup. The user should enable the Data Catalog API in the project identified by the `entry_group.name` parameter (see [Data Catalog Resource Project] (https://cloud.google.com/data-catalog/docs/concepts/resource-project) for more information).");
            entry_groups2 = entry_groups2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy for a resource. Replaces any existing policy. Supported resources are: - Tag templates. - Entries. - Entry groups. Note, this method cannot be used to manage policies for BigQuery, Pub/Sub and any external Google Cloud Platform resources synced to Data Catalog. Callers must have following Google IAM permission - `datacatalog.tagTemplates.setIamPolicy` to set policies on tag templates. - `datacatalog.entries.setIamPolicy` to set policies on entries. - `datacatalog.entryGroups.setIamPolicy` to set policies on entry groups.");
            entry_groups2 = entry_groups2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns the caller\'s permissions on a resource. If the resource does not exist, an empty set of permissions is returned (We don\'t return a `NOT_FOUND` error). Supported resources are: - Tag templates. - Entries. - Entry groups. Note, this method cannot be used to manage policies for BigQuery, Pub/Sub and any external Google Cloud Platform resources synced to Data Catalog. A caller is not required to have Google IAM permission to make this request.");
            entry_groups2 = entry_groups2.subcommand(mcmd);
        }
        let mut tag_templates2 = SubCommand::with_name("tag_templates")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_iam_policy, patch, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a tag template. The user should enable the Data Catalog API in the project identified by the `parent` parameter (see [Data Catalog Resource Project](https://cloud.google.com/data-catalog/docs/concepts/resource-project) for more information).");
            tag_templates2 = tag_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a tag template and all tags using the template. Users should enable the Data Catalog API in the project identified by the `name` parameter (see [Data Catalog Resource Project] (https://cloud.google.com/data-catalog/docs/concepts/resource-project) for more information).");
            tag_templates2 = tag_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a tag template.");
            tag_templates2 = tag_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. A `NOT_FOUND` error is returned if the resource does not exist. An empty policy is returned if the resource exists but does not have a policy set on it. Supported resources are: - Tag templates. - Entries. - Entry groups. Note, this method cannot be used to manage policies for BigQuery, Pub/Sub and any external Google Cloud Platform resources synced to Data Catalog. Callers must have following Google IAM permission - `datacatalog.tagTemplates.getIamPolicy` to get policies on tag templates. - `datacatalog.entries.getIamPolicy` to get policies on entries. - `datacatalog.entryGroups.getIamPolicy` to get policies on entry groups.");
            tag_templates2 = tag_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a tag template. This method cannot be used to update the fields of a template. The tag template fields are represented as separate resources and should be updated using their own create/update/delete methods. Users should enable the Data Catalog API in the project identified by the `tag_template.name` parameter (see [Data Catalog Resource Project] (https://cloud.google.com/data-catalog/docs/concepts/resource-project) for more information).");
            tag_templates2 = tag_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy for a resource. Replaces any existing policy. Supported resources are: - Tag templates. - Entries. - Entry groups. Note, this method cannot be used to manage policies for BigQuery, Pub/Sub and any external Google Cloud Platform resources synced to Data Catalog. Callers must have following Google IAM permission - `datacatalog.tagTemplates.setIamPolicy` to set policies on tag templates. - `datacatalog.entries.setIamPolicy` to set policies on entries. - `datacatalog.entryGroups.setIamPolicy` to set policies on entry groups.");
            tag_templates2 = tag_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns the caller\'s permissions on a resource. If the resource does not exist, an empty set of permissions is returned (We don\'t return a `NOT_FOUND` error). Supported resources are: - Tag templates. - Entries. - Entry groups. Note, this method cannot be used to manage policies for BigQuery, Pub/Sub and any external Google Cloud Platform resources synced to Data Catalog. A caller is not required to have Google IAM permission to make this request.");
            tag_templates2 = tag_templates2.subcommand(mcmd);
        }
        let mut taxonomies2 = SubCommand::with_name("taxonomies")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, export, get, get_iam_policy, import, list, patch, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a taxonomy in the specified project.");
            taxonomies2 = taxonomies2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a taxonomy. This operation will also delete all policy tags in this taxonomy along with their associated policies.");
            taxonomies2 = taxonomies2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("export").about("Exports all taxonomies and their policy tags in a project. This method generates SerializedTaxonomy protos with nested policy tags that can be used as an input for future ImportTaxonomies calls.");
            taxonomies2 = taxonomies2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a taxonomy.");
            taxonomies2 = taxonomies2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy")
                .about("Gets the IAM policy for a taxonomy or a policy tag.");
            taxonomies2 = taxonomies2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("import").about("Imports all taxonomies and their policy tags to a project as new taxonomies. This method provides a bulk taxonomy / policy tag creation using nested proto structure.");
            taxonomies2 = taxonomies2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all taxonomies in a project in a particular location that the caller has permission to view.");
            taxonomies2 = taxonomies2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a taxonomy.");
            taxonomies2 = taxonomies2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy")
                .about("Sets the IAM policy for a taxonomy or a policy tag.");
            taxonomies2 = taxonomies2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns the permissions that a caller has on the specified taxonomy or policy tag.");
            taxonomies2 = taxonomies2.subcommand(mcmd);
        }
        let mut entries3 = SubCommand::with_name("entries")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_iam_policy, list, patch and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create").about("Creates an entry. Only entries of \'FILESET\' type or user-specified type can be created. Users should enable the Data Catalog API in the project identified by the `parent` parameter (see [Data Catalog Resource Project] (https://cloud.google.com/data-catalog/docs/concepts/resource-project) for more information). A maximum of 100,000 entries may be created per entry group.");
            entries3 = entries3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an existing entry. Only entries created through CreateEntry method can be deleted. Users should enable the Data Catalog API in the project identified by the `name` parameter (see [Data Catalog Resource Project] (https://cloud.google.com/data-catalog/docs/concepts/resource-project) for more information).");
            entries3 = entries3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets an entry.");
            entries3 = entries3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. A `NOT_FOUND` error is returned if the resource does not exist. An empty policy is returned if the resource exists but does not have a policy set on it. Supported resources are: - Tag templates. - Entries. - Entry groups. Note, this method cannot be used to manage policies for BigQuery, Pub/Sub and any external Google Cloud Platform resources synced to Data Catalog. Callers must have following Google IAM permission - `datacatalog.tagTemplates.getIamPolicy` to get policies on tag templates. - `datacatalog.entries.getIamPolicy` to get policies on entries. - `datacatalog.entryGroups.getIamPolicy` to get policies on entry groups.");
            entries3 = entries3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists entries.");
            entries3 = entries3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an existing entry. Users should enable the Data Catalog API in the project identified by the `entry.name` parameter (see [Data Catalog Resource Project] (https://cloud.google.com/data-catalog/docs/concepts/resource-project) for more information).");
            entries3 = entries3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns the caller\'s permissions on a resource. If the resource does not exist, an empty set of permissions is returned (We don\'t return a `NOT_FOUND` error). Supported resources are: - Tag templates. - Entries. - Entry groups. Note, this method cannot be used to manage policies for BigQuery, Pub/Sub and any external Google Cloud Platform resources synced to Data Catalog. A caller is not required to have Google IAM permission to make this request.");
            entries3 = entries3.subcommand(mcmd);
        }
        let mut tags3 = SubCommand::with_name("tags")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a tag on an Entry. Note: The project identified by the `parent` parameter for the [tag](https://cloud.google.com/data-catalog/docs/reference/rest/v1beta1/projects.locations.entryGroups.entries.tags/create#path-parameters) and the [tag template](https://cloud.google.com/data-catalog/docs/reference/rest/v1beta1/projects.locations.tagTemplates/create#path-parameters) used to create the tag must be from the same organization.");
            tags3 = tags3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a tag.");
            tags3 = tags3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the tags on an Entry.");
            tags3 = tags3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an existing tag.");
            tags3 = tags3.subcommand(mcmd);
        }
        let mut fields3 = SubCommand::with_name("fields")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, patch and rename");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a field in a tag template. The user should enable the Data Catalog API in the project identified by the `parent` parameter (see [Data Catalog Resource Project](https://cloud.google.com/data-catalog/docs/concepts/resource-project) for more information).");
            fields3 = fields3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a field in a tag template and all uses of that field. Users should enable the Data Catalog API in the project identified by the `name` parameter (see [Data Catalog Resource Project] (https://cloud.google.com/data-catalog/docs/concepts/resource-project) for more information).");
            fields3 = fields3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a field in a tag template. This method cannot be used to update the field type. Users should enable the Data Catalog API in the project identified by the `name` parameter (see [Data Catalog Resource Project] (https://cloud.google.com/data-catalog/docs/concepts/resource-project) for more information).");
            fields3 = fields3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("rename").about("Renames a field in a tag template. The user should enable the Data Catalog API in the project identified by the `name` parameter (see [Data Catalog Resource Project](https://cloud.google.com/data-catalog/docs/concepts/resource-project) for more information).");
            fields3 = fields3.subcommand(mcmd);
        }
        let mut policy_tags3 = SubCommand::with_name("policy_tags")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_iam_policy, list, patch, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a policy tag in the specified taxonomy.");
            policy_tags3 = policy_tags3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes a policy tag. Also deletes all of its descendant policy tags.");
            policy_tags3 = policy_tags3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a policy tag.");
            policy_tags3 = policy_tags3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy")
                .about("Gets the IAM policy for a taxonomy or a policy tag.");
            policy_tags3 = policy_tags3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all policy tags in a taxonomy.");
            policy_tags3 = policy_tags3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a policy tag.");
            policy_tags3 = policy_tags3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy")
                .about("Sets the IAM policy for a taxonomy or a policy tag.");
            policy_tags3 = policy_tags3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns the permissions that a caller has on the specified taxonomy or policy tag.");
            policy_tags3 = policy_tags3.subcommand(mcmd);
        }
        let mut tags4 = SubCommand::with_name("tags")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a tag on an Entry. Note: The project identified by the `parent` parameter for the [tag](https://cloud.google.com/data-catalog/docs/reference/rest/v1beta1/projects.locations.entryGroups.entries.tags/create#path-parameters) and the [tag template](https://cloud.google.com/data-catalog/docs/reference/rest/v1beta1/projects.locations.tagTemplates/create#path-parameters) used to create the tag must be from the same organization.");
            tags4 = tags4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a tag.");
            tags4 = tags4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the tags on an Entry.");
            tags4 = tags4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an existing tag.");
            tags4 = tags4.subcommand(mcmd);
        }
        let mut enum_values4 = SubCommand::with_name("enum_values")
            .setting(AppSettings::ColoredHelp)
            .about("methods: rename");
        {
            let mcmd = SubCommand::with_name("rename").about("Renames an enum value in a tag template. The enum values have to be unique within one enum field. Thus, an enum value cannot be renamed with a name used in any other enum value within the same enum field.");
            enum_values4 = enum_values4.subcommand(mcmd);
        }
        fields3 = fields3.subcommand(enum_values4);
        entries3 = entries3.subcommand(tags4);
        taxonomies2 = taxonomies2.subcommand(policy_tags3);
        tag_templates2 = tag_templates2.subcommand(fields3);
        entry_groups2 = entry_groups2.subcommand(tags3);
        entry_groups2 = entry_groups2.subcommand(entries3);
        locations1 = locations1.subcommand(taxonomies2);
        locations1 = locations1.subcommand(tag_templates2);
        locations1 = locations1.subcommand(entry_groups2);
        projects0 = projects0.subcommand(locations1);
        app = app.subcommand(projects0);
        app = app.subcommand(entries0);
        app = app.subcommand(catalog0);

        Self { app }
    }
}
use google_datacatalog1_beta1 as api;

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
