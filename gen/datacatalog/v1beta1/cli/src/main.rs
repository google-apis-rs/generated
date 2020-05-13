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
            .version("0.1.0-20200430")
            .about("A fully managed and highly scalable data discovery and metadata management service.\n")
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
            let mcmd = SubCommand::with_name("search").about("Searches Data Catalog for multiple resources like entries, tags that\nmatch a query.\n\nThis is a custom method\n(https://cloud.google.com/apis/design/custom_methods) and does not return\nthe complete resource, only the resource identifier and high level\nfields. Clients can subsequentally call `Get` methods.\n\nNote that Data Catalog search queries do not guarantee full recall. Query\nresults that match your query may not be returned, even in subsequent\nresult pages. Also note that results returned (and not returned) can vary\nacross repeated search queries.\n\nSee [Data Catalog Search\nSyntax](https://cloud.google.com/data-catalog/docs/how-to/search-reference)\nfor more information.");
            catalog0 = catalog0.subcommand(mcmd);
        }
        let mut entries0 = SubCommand::with_name("entries")
            .setting(AppSettings::ColoredHelp)
            .about("methods: lookup");
        {
            let mcmd = SubCommand::with_name("lookup").about("Get an entry by target resource name. This method allows clients to use\nthe resource name from the source Google Cloud Platform service to get the\nData Catalog Entry.");
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
            let mcmd = SubCommand::with_name("create").about("A maximum of 10,000 entry groups may be created per organization across all\nlocations.\n\nUsers should enable the Data Catalog API in the project identified by\nthe `parent` parameter (see [Data Catalog Resource Project]\n(https://cloud.google.com/data-catalog/docs/concepts/resource-project) for\nmore information).");
            entry_groups2 = entry_groups2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an EntryGroup. Only entry groups that do not contain entries can be\ndeleted. Users should enable the Data Catalog API in the project\nidentified by the `name` parameter (see [Data Catalog Resource Project]\n(https://cloud.google.com/data-catalog/docs/concepts/resource-project) for\nmore information).");
            entry_groups2 = entry_groups2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets an EntryGroup.");
            entry_groups2 = entry_groups2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. A `NOT_FOUND` error\nis returned if the resource does not exist. An empty policy is returned\nif the resource exists but does not have a policy set on it.\n\nSupported resources are:\n  - Tag templates.\n  - Entries.\n  - Entry groups.\nNote, this method cannot be used to manage policies for BigQuery, Pub/Sub\nand any external Google Cloud Platform resources synced to Data Catalog.\n\nCallers must have following Google IAM permission\n  - `datacatalog.tagTemplates.getIamPolicy` to get policies on tag\n    templates.\n  - `datacatalog.entries.getIamPolicy` to get policies on entries.\n  - `datacatalog.entryGroups.getIamPolicy` to get policies on entry groups.");
            entry_groups2 = entry_groups2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists entry groups.");
            entry_groups2 = entry_groups2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an EntryGroup. The user should enable the Data Catalog API in the\nproject identified by the `entry_group.name` parameter (see [Data Catalog\nResource Project]\n(https://cloud.google.com/data-catalog/docs/concepts/resource-project) for\nmore information).");
            entry_groups2 = entry_groups2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy for a resource. Replaces any existing\npolicy.\nSupported resources are:\n  - Tag templates.\n  - Entries.\n  - Entry groups.\nNote, this method cannot be used to manage policies for BigQuery, Pub/Sub\nand any external Google Cloud Platform resources synced to Data Catalog.\n\nCallers must have following Google IAM permission\n  - `datacatalog.tagTemplates.setIamPolicy` to set policies on tag\n    templates.\n  - `datacatalog.entries.setIamPolicy` to set policies on entries.\n  - `datacatalog.entryGroups.setIamPolicy` to set policies on entry groups.");
            entry_groups2 = entry_groups2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns the caller\'s permissions on a resource.\nIf the resource does not exist, an empty set of permissions is returned\n(We don\'t return a `NOT_FOUND` error).\n\nSupported resources are:\n  - Tag templates.\n  - Entries.\n  - Entry groups.\nNote, this method cannot be used to manage policies for BigQuery, Pub/Sub\nand any external Google Cloud Platform resources synced to Data Catalog.\n\nA caller is not required to have Google IAM permission to make this\nrequest.");
            entry_groups2 = entry_groups2.subcommand(mcmd);
        }
        let mut tag_templates2 = SubCommand::with_name("tag_templates")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_iam_policy, patch, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a tag template. The user should enable the Data Catalog API in\nthe project identified by the `parent` parameter (see [Data Catalog\nResource\nProject](https://cloud.google.com/data-catalog/docs/concepts/resource-project)\nfor more information).");
            tag_templates2 = tag_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a tag template and all tags using the template.\nUsers should enable the Data Catalog API in the project identified by\nthe `name` parameter (see [Data Catalog Resource Project]\n(https://cloud.google.com/data-catalog/docs/concepts/resource-project) for\nmore information).");
            tag_templates2 = tag_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a tag template.");
            tag_templates2 = tag_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. A `NOT_FOUND` error\nis returned if the resource does not exist. An empty policy is returned\nif the resource exists but does not have a policy set on it.\n\nSupported resources are:\n  - Tag templates.\n  - Entries.\n  - Entry groups.\nNote, this method cannot be used to manage policies for BigQuery, Pub/Sub\nand any external Google Cloud Platform resources synced to Data Catalog.\n\nCallers must have following Google IAM permission\n  - `datacatalog.tagTemplates.getIamPolicy` to get policies on tag\n    templates.\n  - `datacatalog.entries.getIamPolicy` to get policies on entries.\n  - `datacatalog.entryGroups.getIamPolicy` to get policies on entry groups.");
            tag_templates2 = tag_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a tag template. This method cannot be used to update the fields of\na template. The tag template fields are represented as separate resources\nand should be updated using their own create/update/delete methods.\nUsers should enable the Data Catalog API in the project identified by\nthe `tag_template.name` parameter (see [Data Catalog Resource Project]\n(https://cloud.google.com/data-catalog/docs/concepts/resource-project) for\nmore information).");
            tag_templates2 = tag_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy for a resource. Replaces any existing\npolicy.\nSupported resources are:\n  - Tag templates.\n  - Entries.\n  - Entry groups.\nNote, this method cannot be used to manage policies for BigQuery, Pub/Sub\nand any external Google Cloud Platform resources synced to Data Catalog.\n\nCallers must have following Google IAM permission\n  - `datacatalog.tagTemplates.setIamPolicy` to set policies on tag\n    templates.\n  - `datacatalog.entries.setIamPolicy` to set policies on entries.\n  - `datacatalog.entryGroups.setIamPolicy` to set policies on entry groups.");
            tag_templates2 = tag_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns the caller\'s permissions on a resource.\nIf the resource does not exist, an empty set of permissions is returned\n(We don\'t return a `NOT_FOUND` error).\n\nSupported resources are:\n  - Tag templates.\n  - Entries.\n  - Entry groups.\nNote, this method cannot be used to manage policies for BigQuery, Pub/Sub\nand any external Google Cloud Platform resources synced to Data Catalog.\n\nA caller is not required to have Google IAM permission to make this\nrequest.");
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
            let mcmd = SubCommand::with_name("delete").about("Deletes a taxonomy. This operation will also delete all\npolicy tags in this taxonomy along with their associated policies.");
            taxonomies2 = taxonomies2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("export").about("Exports all taxonomies and their policy tags in a project.\n\nThis method generates SerializedTaxonomy protos with nested policy tags\nthat can be used as an input for future ImportTaxonomies calls.");
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
            let mcmd = SubCommand::with_name("import").about("Imports all taxonomies and their policy tags to a project as new\ntaxonomies.\n\nThis method provides a bulk taxonomy / policy tag creation using nested\nproto structure.");
            taxonomies2 = taxonomies2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all taxonomies in a project in a particular location that the caller\nhas permission to view.");
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
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns the permissions that a caller has on the specified taxonomy or\npolicy tag.");
            taxonomies2 = taxonomies2.subcommand(mcmd);
        }
        let mut entries3 = SubCommand::with_name("entries")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_iam_policy, list, patch and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create").about("Creates an entry. Only entries of \'FILESET\' type or user-specified type can\nbe created.\n\nUsers should enable the Data Catalog API in the project identified by\nthe `parent` parameter (see [Data Catalog Resource Project]\n(https://cloud.google.com/data-catalog/docs/concepts/resource-project) for\nmore information).\n\nA maximum of 100,000 entries may be created per entry group.");
            entries3 = entries3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an existing entry. Only entries created through\nCreateEntry\nmethod can be deleted.\nUsers should enable the Data Catalog API in the project identified by\nthe `name` parameter (see [Data Catalog Resource Project]\n(https://cloud.google.com/data-catalog/docs/concepts/resource-project) for\nmore information).");
            entries3 = entries3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets an entry.");
            entries3 = entries3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. A `NOT_FOUND` error\nis returned if the resource does not exist. An empty policy is returned\nif the resource exists but does not have a policy set on it.\n\nSupported resources are:\n  - Tag templates.\n  - Entries.\n  - Entry groups.\nNote, this method cannot be used to manage policies for BigQuery, Pub/Sub\nand any external Google Cloud Platform resources synced to Data Catalog.\n\nCallers must have following Google IAM permission\n  - `datacatalog.tagTemplates.getIamPolicy` to get policies on tag\n    templates.\n  - `datacatalog.entries.getIamPolicy` to get policies on entries.\n  - `datacatalog.entryGroups.getIamPolicy` to get policies on entry groups.");
            entries3 = entries3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists entries.");
            entries3 = entries3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an existing entry.\nUsers should enable the Data Catalog API in the project identified by\nthe `entry.name` parameter (see [Data Catalog Resource Project]\n(https://cloud.google.com/data-catalog/docs/concepts/resource-project) for\nmore information).");
            entries3 = entries3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns the caller\'s permissions on a resource.\nIf the resource does not exist, an empty set of permissions is returned\n(We don\'t return a `NOT_FOUND` error).\n\nSupported resources are:\n  - Tag templates.\n  - Entries.\n  - Entry groups.\nNote, this method cannot be used to manage policies for BigQuery, Pub/Sub\nand any external Google Cloud Platform resources synced to Data Catalog.\n\nA caller is not required to have Google IAM permission to make this\nrequest.");
            entries3 = entries3.subcommand(mcmd);
        }
        let mut tags3 = SubCommand::with_name("tags")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a tag on an Entry.\nNote: The project identified by the `parent` parameter for the\n[tag](https://cloud.google.com/data-catalog/docs/reference/rest/v1beta1/projects.locations.entryGroups.entries.tags/create#path-parameters)\nand the\n[tag\ntemplate](https://cloud.google.com/data-catalog/docs/reference/rest/v1beta1/projects.locations.tagTemplates/create#path-parameters)\nused to create the tag must be from the same organization.");
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
            let mcmd = SubCommand::with_name("create").about("Creates a field in a tag template. The user should enable the Data Catalog\nAPI in the project identified by the `parent` parameter (see\n[Data Catalog Resource\nProject](https://cloud.google.com/data-catalog/docs/concepts/resource-project)\nfor more information).");
            fields3 = fields3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a field in a tag template and all uses of that field.\nUsers should enable the Data Catalog API in the project identified by\nthe `name` parameter (see [Data Catalog Resource Project]\n(https://cloud.google.com/data-catalog/docs/concepts/resource-project) for\nmore information).");
            fields3 = fields3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a field in a tag template. This method cannot be used to update the\nfield type. Users should enable the Data Catalog API in the project\nidentified by the `name` parameter (see [Data Catalog Resource Project]\n(https://cloud.google.com/data-catalog/docs/concepts/resource-project) for\nmore information).");
            fields3 = fields3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("rename").about("Renames a field in a tag template. The user should enable the Data Catalog\nAPI in the project identified by the `name` parameter (see [Data Catalog\nResource\nProject](https://cloud.google.com/data-catalog/docs/concepts/resource-project)\nfor more information).");
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
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns the permissions that a caller has on the specified taxonomy or\npolicy tag.");
            policy_tags3 = policy_tags3.subcommand(mcmd);
        }
        let mut tags4 = SubCommand::with_name("tags")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a tag on an Entry.\nNote: The project identified by the `parent` parameter for the\n[tag](https://cloud.google.com/data-catalog/docs/reference/rest/v1beta1/projects.locations.entryGroups.entries.tags/create#path-parameters)\nand the\n[tag\ntemplate](https://cloud.google.com/data-catalog/docs/reference/rest/v1beta1/projects.locations.tagTemplates/create#path-parameters)\nused to create the tag must be from the same organization.");
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
