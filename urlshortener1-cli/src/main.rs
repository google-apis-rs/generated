use clap::{App, Arg, SubCommand};
use std::{
    io::{self, Write},
    path::PathBuf,
};

use google_urlshortener1 as api;
use hyper_rustls::HttpsConnector;

use google_cli_shared::*;

use clap::ArgMatches;
use serde_json as json;
use yup_oauth2::Authenticator;

fn main() {
    let mut exit_status = 0i32;
    let arg_data = [
        ("url", "methods: 'get', 'insert' and 'list'", vec![
            ("get",
             Some(r##"Expands a short URL or gets creation time and analytics."##),
             "Details at http://byron.github.io/google-apis-rs/google_urlshortener1_cli/url_get",
             vec![
                 (Some(r##"short-url"##),
                  None,
                  Some(r##"The short URL, including the protocol."##),
                  Some(true),
                  Some(false)),
                 (Some(r##"v"##),
                  Some(r##"p"##),
                  Some(r##"Set various optional parameters, matching the key=value form"##),
                  Some(false),
                  Some(true)),
                 (Some(r##"out"##),
                  Some(r##"o"##),
                  Some(r##"Specify the file into which to write the program's output"##),
                  Some(false),
                  Some(false)),
             ]),
            ("insert",
             Some(r##"Creates a new short URL."##),
             "Details at http://byron.github.io/google-apis-rs/google_urlshortener1_cli/url_insert",
             vec![
                 (Some(r##"kv"##),
                  Some(r##"r"##),
                  Some(r##"Set various fields of the request structure, matching the key=value form"##),
                  Some(true),
                  Some(true)),
                 (Some(r##"v"##),
                  Some(r##"p"##),
                  Some(r##"Set various optional parameters, matching the key=value form"##),
                  Some(false),
                  Some(true)),
                 (Some(r##"out"##),
                  Some(r##"o"##),
                  Some(r##"Specify the file into which to write the program's output"##),
                  Some(false),
                  Some(false)),
             ]),
            ("list",
             Some(r##"Retrieves a list of URLs shortened by a user."##),
             "Details at http://byron.github.io/google-apis-rs/google_urlshortener1_cli/url_list",
             vec![
                 (Some(r##"v"##),
                  Some(r##"p"##),
                  Some(r##"Set various optional parameters, matching the key=value form"##),
                  Some(false),
                  Some(true)),
                 (Some(r##"out"##),
                  Some(r##"o"##),
                  Some(r##"Specify the file into which to write the program's output"##),
                  Some(false),
                  Some(false)),
             ]),
        ]),
    ];

    let mut app = App::new("urlshortener1")
        .author("Sebastian Thiel <byronimo@gmail.com>")
        .version("1.0.10+20150519")
        .about("Lets you create, inspect, and manage goo.gl short URLs")
        .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_urlshortener1_cli")
        .arg(Arg::with_name("scope")
            .long("scope")
            .help("Specify the authentication method should be executed in. Each scope requires the user to grant this application permission to use it.If unset, it defaults to the shortest scope url for a particular method.")
            .multiple(true)
            .takes_value(true))
        .arg(Arg::with_name("folder")
            .long("config-dir")
            .help("A directory into which we will store our persistent data. Defaults to a user-writable directory that we will create during the first invocation.[default: ~/.google-service-cli")
            .multiple(false)
            .takes_value(true))
        .arg(Arg::with_name("debug")
            .long("debug")
            .help("Provide more output to aid with debugging")
            .multiple(false)
            .takes_value(false));

    for &(main_command_name, about, ref subcommands) in arg_data.iter() {
        let mut mcmd = SubCommand::with_name(main_command_name).about(about);

        for &(sub_command_name, ref desc, url_info, ref args) in subcommands {
            let mut scmd = SubCommand::with_name(sub_command_name);
            if let Some(desc) = *desc {
                scmd = scmd.about(desc);
            }
            scmd = scmd.after_help(url_info);

            for &(ref arg_name, ref flag, ref desc, ref required, ref multi) in args {
                let arg_name_str = match (arg_name, flag) {
                    (&Some(an), _) => an,
                    (_, &Some(f)) => f,
                    _ => unreachable!(),
                };
                let mut arg = Arg::with_name(arg_name_str).empty_values(false);
                if let Some(short_flag) = flag {
                    arg = arg.short(short_flag);
                }
                if let Some(desc) = desc {
                    arg = arg.help(desc);
                }
                if arg_name.is_some() && flag.is_some() {
                    arg = arg.takes_value(true);
                }
                if let Some(required) = *required {
                    arg = arg.required(required);
                }
                if let Some(multi) = *multi {
                    arg = arg.multiple(multi);
                }
                scmd = scmd.arg(arg);
            }
            mcmd = mcmd.subcommand(scmd);
        }
        app = app.subcommand(mcmd);
    }

    let matches = app.get_matches();
    let debug = matches.is_present("debug");
    match new(matches) {
        Err(err) => {
            exit_status = err.exit_code;
            writeln!(io::stderr(), "{}", err).ok();
        }
        Ok((opt, client)) => {
            if let Err(err) = execute(&client, &opt, false).expect("no failure should be possible")
            {
                exit_status = 1;
                match err {
                    ExecuteError::IoError(path, err) => {
                        writeln!(
                            io::stderr(),
                            "Failed to open output file '{}': {}",
                            path,
                            err
                        )
                        .ok();
                    }
                    ExecuteError::ApiError(err) => {
                        if debug {
                            writeln!(io::stderr(), "{:#?}", err).ok();
                        } else {
                            writeln!(io::stderr(), "{}", err).ok();
                        }
                    }
                }
            }
        }
    }

    std::process::exit(exit_status);
}

const GP: [&str; 7] = [
    "alt",
    "fields",
    "key",
    "oauth-token",
    "pretty-print",
    "quota-user",
    "user-ip",
];

const _GPM: [(&str, &str); 4] = [
    ("oauth-token", "oauth_token"),
    ("pretty-print", "prettyPrint"),
    ("quota-user", "quotaUser"),
    ("user-ip", "userIp"),
];

fn new(opt: ArgMatches) -> Result<(ArgMatches, api::Client), InvalidOptionsError> {
    let (config_dir, secret) = {
        let config_dir = match assure_config_dir_exists(
            opt.value_of("folder").unwrap_or("~/.google-service-cli"),
        ) {
            Err(e) => return Err(InvalidOptionsError::single(e, 3)),
            Ok(p) => p,
        };

        match application_secret_from_directory(&config_dir, "urlshortener1-secret.json") {
            Ok(secret) => (config_dir, secret),
            Err(e) => return Err(InvalidOptionsError::single(e, 4)),
        }
    };

    let https = HttpsConnector::new(1);
    let client = hyper::Client::builder()
        .keep_alive(false)
        .build::<_, hyper::Body>(https);

    // InstalledFlow handles OAuth flows of that type. They are usually the ones where a user
    // grants access to their personal account (think Google Drive, Github API, etc.).
    let inf = yup_oauth2::InstalledFlow::new(
        secret,
        yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect(8081),
    );

    let auth = Authenticator::new(inf)
        .persist_tokens_to_disk(PathBuf::from(config_dir).join("tokens.json"))
        .hyper_client(client)
        .build()
        .expect("create a new statically known client");

    let auth = google_api_auth::yup_oauth2::from_authenticator(
        auth,
        opt.values_of("scope")
            .map(|i| i.map(String::from).collect::<Vec<_>>())
            .unwrap_or_else(|| vec!["https://www.googleapis.com/auth/urlshortener".into()]),
    );

    let client = api::Client::new(auth);
    match execute(&client, &opt, true) {
        Err(Some(err)) => Err(err),
        Err(None) => Ok((opt, client)),
        Ok(_) => {
            unreachable!("dry runs are never successful, right now. TODO: can this be different?")
        }
    }
}

fn url_get(
    hub: &api::Client,
    opt: &ArgMatches,
    dry_run: bool,
    err: &mut InvalidOptionsError,
) -> Result<(), ExecuteError> {
    let keep = hub.url();
    let mut call = keep.get(opt.value_of("short-url").unwrap_or(""));
    for parg in opt
        .values_of("v")
        .map(|i| i.collect())
        .unwrap_or_else(Vec::new)
        .iter()
    {
        let (key, value) = parse_kv_arg(&*parg, err, false);
        match key {
            "projection" => {
                call = call.projection(json::from_str(value.unwrap_or(""))?);
            }
            "key" => call = call.key(value.unwrap_or("")),
            _ => {
                err.issues
                    .push(CLIError::UnknownParameter(key.to_string(), {
                        let mut v = Vec::new();
                        v.extend(GP.iter().copied());
                        v.extend(["projection"].iter().copied());
                        v
                    }));
            }
        }
    }
    let protocol = CallType::Standard;
    if dry_run {
        Ok(())
    } else {
        assert!(err.issues.is_empty());
        let mut ostream = match writer_from_opts(opt.value_of("out")) {
            Ok(f) => f,
            Err(io_err) => {
                return Err(ExecuteError::IoError(
                    opt.value_of("out").unwrap_or("-").to_string(),
                    io_err,
                ));
            }
        };
        match match protocol {
            CallType::Standard => call.execute_with_all_fields(),
            _ => unreachable!(),
        } {
            Err(api_err) => Err(api_err.into()),
            Ok(response) => {
                json::to_writer_pretty(&mut ostream, &response).unwrap();
                ostream.flush().unwrap();
                Ok(())
            }
        }
    }
}

fn execute(
    hub: &api::Client,
    opt: &ArgMatches,
    dry_run: bool,
) -> Result<Result<(), ExecuteError>, Option<InvalidOptionsError>> {
    let mut err = InvalidOptionsError::new();
    let mut call_result: Result<(), ExecuteError> = Ok(());
    let mut err_opt: Option<InvalidOptionsError> = None;
    match opt.subcommand() {
        ("url", Some(opt)) => match opt.subcommand() {
            ("get", Some(opt)) => {
                call_result = url_get(hub, opt, dry_run, &mut err);
            }
            ("insert", Some(opt)) => {
                call_result = url_insert(hub, opt, dry_run, &mut err);
            }
            ("list", Some(opt)) => {
                call_result = url_list(hub, opt, dry_run, &mut err);
            }
            _ => {
                err.issues
                    .push(CLIError::MissingMethodError("url".to_string()));
                writeln!(io::stderr(), "{}\n", opt.usage()).ok();
            }
        },
        _ => {
            err.issues.push(CLIError::MissingCommandError);
            writeln!(io::stderr(), "{}\n", opt.usage()).ok();
        }
    }

    if dry_run {
        if !err.issues.is_empty() {
            err_opt = Some(err);
        }
        Err(err_opt)
    } else {
        assert!(err.issues.is_empty());
        Ok(call_result)
    }
}

fn url_insert(
    hub: &api::Client,
    opt: &ArgMatches,
    dry_run: bool,
    err: &mut InvalidOptionsError,
) -> Result<(), ExecuteError> {
    let object = object_from_kvargs(
        opt.values_of("kv")
            .map(|i| i.collect())
            .unwrap_or_else(Vec::new),
        err,
        &[
            (
                "status",
                "status",
                JsonTypeInfo {
                    jtype: JsonType::String,
                    ctype: ComplexType::Pod,
                },
            ),
            (
                "kind",
                "kind",
                JsonTypeInfo {
                    jtype: JsonType::String,
                    ctype: ComplexType::Pod,
                },
            ),
            (
                "created",
                "created",
                JsonTypeInfo {
                    jtype: JsonType::String,
                    ctype: ComplexType::Pod,
                },
            ),
            (
                "analytics.week.short-url-clicks",
                "analytics.week.shortUrlClicks",
                JsonTypeInfo {
                    jtype: JsonType::String,
                    ctype: ComplexType::Pod,
                },
            ),
            (
                "analytics.week.long-url-clicks",
                "analytics.week.longUrlClicks",
                JsonTypeInfo {
                    jtype: JsonType::String,
                    ctype: ComplexType::Pod,
                },
            ),
            (
                "analytics.all-time.short-url-clicks",
                "analytics.allTime.shortUrlClicks",
                JsonTypeInfo {
                    jtype: JsonType::String,
                    ctype: ComplexType::Pod,
                },
            ),
            (
                "analytics.all-time.long-url-clicks",
                "analytics.allTime.longUrlClicks",
                JsonTypeInfo {
                    jtype: JsonType::String,
                    ctype: ComplexType::Pod,
                },
            ),
            (
                "analytics.two-hours.short-url-clicks",
                "analytics.twoHours.shortUrlClicks",
                JsonTypeInfo {
                    jtype: JsonType::String,
                    ctype: ComplexType::Pod,
                },
            ),
            (
                "analytics.two-hours.long-url-clicks",
                "analytics.twoHours.longUrlClicks",
                JsonTypeInfo {
                    jtype: JsonType::String,
                    ctype: ComplexType::Pod,
                },
            ),
            (
                "analytics.day.short-url-clicks",
                "analytics.day.shortUrlClicks",
                JsonTypeInfo {
                    jtype: JsonType::String,
                    ctype: ComplexType::Pod,
                },
            ),
            (
                "analytics.day.long-url-clicks",
                "analytics.day.longUrlClicks",
                JsonTypeInfo {
                    jtype: JsonType::String,
                    ctype: ComplexType::Pod,
                },
            ),
            (
                "analytics.month.short-url-clicks",
                "analytics.month.shortUrlClicks",
                JsonTypeInfo {
                    jtype: JsonType::String,
                    ctype: ComplexType::Pod,
                },
            ),
            (
                "analytics.month.long-url-clicks",
                "analytics.month.longUrlClicks",
                JsonTypeInfo {
                    jtype: JsonType::String,
                    ctype: ComplexType::Pod,
                },
            ),
            (
                "long-url",
                "longUrl",
                JsonTypeInfo {
                    jtype: JsonType::String,
                    ctype: ComplexType::Pod,
                },
            ),
            (
                "id",
                "id",
                JsonTypeInfo {
                    jtype: JsonType::String,
                    ctype: ComplexType::Pod,
                },
            ),
        ],
    );

    let request: api::schemas::Url = json::value::from_value(object).unwrap();
    let keep = hub.url();
    let call = keep.insert(request);
    for parg in opt
        .values_of("v")
        .map(|i| i.collect())
        .unwrap_or_else(Vec::new)
        .iter()
    {
        let (key, _value) = parse_kv_arg(&*parg, err, false);
        match key {
            _ => {
                let found = false;
                // TODO: params
                // for param in &GP {
                //     if key == *param {
                //         found = true;
                //         call = call.param(GPM.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                //         break;
                //     }
                // }
                if !found {
                    err.issues
                        .push(CLIError::UnknownParameter(key.to_string(), {
                            let mut v = Vec::new();
                            v.extend(GP.iter().copied());
                            v
                        }));
                }
            }
        }
    }
    let protocol = CallType::Standard;
    if dry_run {
        Ok(())
    } else {
        assert!(err.issues.is_empty());
        let mut ostream = match writer_from_opts(opt.value_of("out")) {
            Ok(f) => f,
            Err(io_err) => {
                return Err(ExecuteError::IoError(
                    opt.value_of("out").unwrap_or("-").to_string(),
                    io_err,
                ));
            }
        };
        match match protocol {
            CallType::Standard => call.execute_with_all_fields(),
            _ => unreachable!(),
        } {
            Err(api_err) => Err(api_err.into()),
            Ok(response) => {
                json::to_writer_pretty(&mut ostream, &response).unwrap();
                ostream.flush().unwrap();
                Ok(())
            }
        }
    }
}

fn url_list(
    hub: &api::Client,
    opt: &ArgMatches,
    dry_run: bool,
    err: &mut InvalidOptionsError,
) -> Result<(), ExecuteError> {
    let keep = hub.url();
    let mut call = keep.list();
    for parg in opt
        .values_of("v")
        .map(|i| i.collect())
        .unwrap_or_else(Vec::new)
        .iter()
    {
        let (key, value) = parse_kv_arg(&*parg, err, false);
        match key {
            "start-token" => call = call.start_token(value.unwrap_or("")),
            "projection" => call = call.projection(json::from_str(value.unwrap_or(""))?),
            "key" => call = call.key(value.unwrap_or("")),
            _ => {
                err.issues
                    .push(CLIError::UnknownParameter(key.to_string(), {
                        let mut v = Vec::new();
                        v.extend(GP.iter().copied());
                        v.extend(["start-token", "projection"].iter().copied());
                        v
                    }));
            }
        }
    }
    let protocol = CallType::Standard;
    if dry_run {
        Ok(())
    } else {
        assert!(err.issues.is_empty());
        let mut ostream = match writer_from_opts(opt.value_of("out")) {
            Ok(f) => f,
            Err(io_err) => {
                return Err(ExecuteError::IoError(
                    opt.value_of("out").unwrap_or("-").to_string(),
                    io_err,
                ));
            }
        };
        match match protocol {
            CallType::Standard => call.execute_with_all_fields(),
            _ => unreachable!(),
        } {
            Err(api_err) => Err(api_err.into()),
            Ok(response) => {
                json::to_writer_pretty(&mut ostream, &response).unwrap();
                ostream.flush().unwrap();
                Ok(())
            }
        }
    }
}
