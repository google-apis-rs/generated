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
        let mut app = App::new("cloudkms1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200409")
            .about("Manages keys and performs cryptographic operations in a central cloud service, for direct use by other cloud resources and applications.\n")
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
        let mut key_rings2 = SubCommand::with_name("key_rings")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, get, get_iam_policy, list, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Create a new KeyRing in a given Project and Location.");
            key_rings2 = key_rings2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns metadata for a given KeyRing.");
            key_rings2 = key_rings2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource.\nReturns an empty policy if the resource exists and does not have a policy\nset.");
            key_rings2 = key_rings2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists KeyRings.");
            key_rings2 = key_rings2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any\nexisting policy.\n\nCan return Public Errors: NOT_FOUND, INVALID_ARGUMENT and PERMISSION_DENIED");
            key_rings2 = key_rings2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource.\nIf the resource does not exist, this will return an empty set of\npermissions, not a NOT_FOUND error.\n\nNote: This operation is designed to be used for building permission-aware\nUIs and command-line tools, not for authorization checking. This operation\nmay \"fail open\" without warning.");
            key_rings2 = key_rings2.subcommand(mcmd);
        }
        let mut crypto_keys3 = SubCommand::with_name("crypto_keys")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, decrypt, encrypt, get, get_iam_policy, list, patch, set_iam_policy, test_iam_permissions and update_primary_version");
        {
            let mcmd = SubCommand::with_name("create").about("Create a new CryptoKey within a KeyRing.\n\nCryptoKey.purpose and\nCryptoKey.version_template.algorithm\nare required.");
            crypto_keys3 = crypto_keys3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("decrypt").about("Decrypts data that was protected by Encrypt. The CryptoKey.purpose\nmust be ENCRYPT_DECRYPT.");
            crypto_keys3 = crypto_keys3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("encrypt").about("Encrypts data, so that it can only be recovered by a call to Decrypt.\nThe CryptoKey.purpose must be\nENCRYPT_DECRYPT.");
            crypto_keys3 = crypto_keys3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about(
                "Returns metadata for a given CryptoKey, as well as its\nprimary CryptoKeyVersion.",
            );
            crypto_keys3 = crypto_keys3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource.\nReturns an empty policy if the resource exists and does not have a policy\nset.");
            crypto_keys3 = crypto_keys3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists CryptoKeys.");
            crypto_keys3 = crypto_keys3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Update a CryptoKey.");
            crypto_keys3 = crypto_keys3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any\nexisting policy.\n\nCan return Public Errors: NOT_FOUND, INVALID_ARGUMENT and PERMISSION_DENIED");
            crypto_keys3 = crypto_keys3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource.\nIf the resource does not exist, this will return an empty set of\npermissions, not a NOT_FOUND error.\n\nNote: This operation is designed to be used for building permission-aware\nUIs and command-line tools, not for authorization checking. This operation\nmay \"fail open\" without warning.");
            crypto_keys3 = crypto_keys3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_primary_version").about("Update the version of a CryptoKey that will be used in Encrypt.\n\nReturns an error if called on an asymmetric key.");
            crypto_keys3 = crypto_keys3.subcommand(mcmd);
        }
        let mut import_jobs3 = SubCommand::with_name("import_jobs")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, get, get_iam_policy, list, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create").about(
                "Create a new ImportJob within a KeyRing.\n\nImportJob.import_method is required.",
            );
            import_jobs3 = import_jobs3.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Returns metadata for a given ImportJob.");
            import_jobs3 = import_jobs3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource.\nReturns an empty policy if the resource exists and does not have a policy\nset.");
            import_jobs3 = import_jobs3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists ImportJobs.");
            import_jobs3 = import_jobs3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any\nexisting policy.\n\nCan return Public Errors: NOT_FOUND, INVALID_ARGUMENT and PERMISSION_DENIED");
            import_jobs3 = import_jobs3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource.\nIf the resource does not exist, this will return an empty set of\npermissions, not a NOT_FOUND error.\n\nNote: This operation is designed to be used for building permission-aware\nUIs and command-line tools, not for authorization checking. This operation\nmay \"fail open\" without warning.");
            import_jobs3 = import_jobs3.subcommand(mcmd);
        }
        let mut crypto_key_versions4 = SubCommand::with_name("crypto_key_versions")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: asymmetric_decrypt, asymmetric_sign, create, destroy, get, get_public_key, import, list, patch and restore");
        {
            let mcmd = SubCommand::with_name("asymmetric_decrypt").about("Decrypts data that was encrypted with a public key retrieved from\nGetPublicKey corresponding to a CryptoKeyVersion with\nCryptoKey.purpose ASYMMETRIC_DECRYPT.");
            crypto_key_versions4 = crypto_key_versions4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("asymmetric_sign").about("Signs data using a CryptoKeyVersion with CryptoKey.purpose\nASYMMETRIC_SIGN, producing a signature that can be verified with the public\nkey retrieved from GetPublicKey.");
            crypto_key_versions4 = crypto_key_versions4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Create a new CryptoKeyVersion in a CryptoKey.\n\nThe server will assign the next sequential id. If unset,\nstate will be set to\nENABLED.");
            crypto_key_versions4 = crypto_key_versions4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("destroy").about("Schedule a CryptoKeyVersion for destruction.\n\nUpon calling this method, CryptoKeyVersion.state will be set to\nDESTROY_SCHEDULED\nand destroy_time will be set to a time 24\nhours in the future, at which point the state\nwill be changed to\nDESTROYED, and the key\nmaterial will be irrevocably destroyed.\n\nBefore the destroy_time is reached,\nRestoreCryptoKeyVersion may be called to reverse the process.");
            crypto_key_versions4 = crypto_key_versions4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Returns metadata for a given CryptoKeyVersion.");
            crypto_key_versions4 = crypto_key_versions4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_public_key").about("Returns the public key for the given CryptoKeyVersion. The\nCryptoKey.purpose must be\nASYMMETRIC_SIGN or\nASYMMETRIC_DECRYPT.");
            crypto_key_versions4 = crypto_key_versions4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("import").about("Imports a new CryptoKeyVersion into an existing CryptoKey using the\nwrapped key material provided in the request.\n\nThe version ID will be assigned the next sequential id within the\nCryptoKey.");
            crypto_key_versions4 = crypto_key_versions4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists CryptoKeyVersions.");
            crypto_key_versions4 = crypto_key_versions4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Update a CryptoKeyVersion\'s metadata.\n\nstate may be changed between\nENABLED and\nDISABLED using this\nmethod. See DestroyCryptoKeyVersion and RestoreCryptoKeyVersion to\nmove between other states.");
            crypto_key_versions4 = crypto_key_versions4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("restore").about("Restore a CryptoKeyVersion in the\nDESTROY_SCHEDULED\nstate.\n\nUpon restoration of the CryptoKeyVersion, state\nwill be set to DISABLED,\nand destroy_time will be cleared.");
            crypto_key_versions4 = crypto_key_versions4.subcommand(mcmd);
        }
        crypto_keys3 = crypto_keys3.subcommand(crypto_key_versions4);
        key_rings2 = key_rings2.subcommand(import_jobs3);
        key_rings2 = key_rings2.subcommand(crypto_keys3);
        locations1 = locations1.subcommand(key_rings2);
        projects0 = projects0.subcommand(locations1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_cloudkms1 as api;

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
