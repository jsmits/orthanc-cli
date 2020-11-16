mod lib;

use clap::{crate_authors, crate_description, crate_version, App, Arg};
use comfy_table::Table;
use lib::*;
use std::env;
use std::process;

fn print(table: Table) {
    println!("{}", table);
}

fn exit_with_error(error: CliError) {
    let output = create_error_table(error);
    eprintln!("{}", output);
    process::exit(1);
}

fn get_server_address(cmd_option: Option<&str>) -> Result<String, CliError> {
    match cmd_option {
        Some(s) => Ok(s.to_string()),
        None => match env::var("ORC_ORTHANC_ADDRESS") {
            Ok(s) => Ok(s),
            Err(e) => Err(CliError::new(
                "Command error",
                Some("Neither --server-address nor ORC_ORTHANC_ADDRESS are set"),
                Some(&format!("{}", e)),
            )),
        },
    }
}

fn get_username(cmd_option: Option<&str>) -> Option<String> {
    match cmd_option {
        Some(s) => Some(s.to_string()),
        None => match env::var("ORC_ORTHANC_USERNAME") {
            Ok(s) => Some(s),
            Err(_) => None, // TODO: This will hide the error
        },
    }
}

fn get_password(cmd_option: Option<&str>) -> Option<String> {
    match cmd_option {
        Some(s) => Some(s.to_string()),
        None => match env::var("ORC_ORTHANC_PASSWORD") {
            Ok(s) => Some(s),
            Err(_) => None, // TODO: This will hide the error
        },
    }
}

fn main() {
    let orthanc = App::new("orthanc")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(clap::AppSettings::SubcommandRequiredElseHelp)
        .arg(
            Arg::new("server")
                .display_order(0)
                .about("Orthanc server address")
                .takes_value(true)
                .short('s')
                .long("server"),
        )
        .arg(
            Arg::new("username")
                .display_order(1)
                .about("Orthanc username")
                .takes_value(true)
                .short('u')
                .long("username"),
        )
        .arg(
            Arg::new("password")
                .display_order(2)
                .about("Orthanc password")
                .takes_value(true)
                .short('p')
                .long("password"),
        )
        .subcommand(
            App::new("patient")
                .setting(clap::AppSettings::SubcommandRequiredElseHelp)
                .display_order(0)
                .about("Patient-level commands")
                .subcommand(App::new("list").display_order(0).about("List all patients"))
                .subcommand(
                    App::new("show")
                        .display_order(1)
                        .about("Show patient details")
                        .arg(Arg::new("id").about("Patient ID").required(true)),
                )
                .subcommand(
                    App::new("anonymize")
                        .display_order(2)
                        .about("Anonymize patient")
                        .arg(Arg::new("id").about("Patient ID").required(true))
                        .arg(
                            Arg::new("config")
                                .about("Anonymization configuration file")
                                .takes_value(true)
                                .short('c')
                                .long("config"),
                        ),
                )
                .subcommand(
                    App::new("modify")
                        .display_order(3)
                        .about("Modify patient")
                        .arg(Arg::new("id").about("Patient ID").required(true))
                        .arg(
                            Arg::new("config")
                                .about("Modification configuration file")
                                .takes_value(true)
                                .short('c')
                                .long("config")
                                .required(true),
                        ),
                )
                .subcommand(
                    App::new("download")
                        .display_order(4)
                        .about("Download patient")
                        .arg(Arg::new("id").about("Patient ID").required(true))
                        .arg(
                            Arg::new("output")
                                .about("Output file path")
                                .takes_value(true)
                                .short('o')
                                .long("output")
                                .required(true),
                        ),
                )
                .subcommand(
                    App::new("delete")
                        .display_order(4)
                        .about("Delete patient")
                        .arg(Arg::new("id").about("Patient ID").required(true)),
                ),
        )
        .subcommand(
            App::new("study")
                .setting(clap::AppSettings::SubcommandRequiredElseHelp)
                .display_order(1)
                .about("Study-level commands")
                .subcommand(
                    App::new("list")
                        .display_order(0)
                        .about("List all studies")
                        .arg(
                            Arg::new("patient_id")
                                .takes_value(true)
                                .short('i')
                                .long("patient-id")
                                .about("Show only studies, belonging to specified patient"),
                        ),
                )
                .subcommand(
                    App::new("show")
                        .display_order(1)
                        .about("Show study details")
                        .arg(Arg::new("id").about("Study ID").required(true)),
                )
                .subcommand(
                    App::new("anonymize")
                        .display_order(2)
                        .about("Anonymize study")
                        .arg(Arg::new("id").about("Study ID").required(true))
                        .arg(
                            Arg::new("config")
                                .about("Anonymization configuration file")
                                .takes_value(true)
                                .short('c')
                                .long("config"),
                        ),
                )
                .subcommand(
                    App::new("modify")
                        .display_order(3)
                        .about("Modify study")
                        .arg(Arg::new("id").about("Study ID").required(true))
                        .arg(
                            Arg::new("config")
                                .about("Modification configuration file")
                                .takes_value(true)
                                .short('c')
                                .long("config")
                                .required(true),
                        ),
                )
                .subcommand(
                    App::new("download")
                        .display_order(4)
                        .about("Download study")
                        .arg(Arg::new("id").about("Study ID").required(true))
                        .arg(
                            Arg::new("output")
                                .about("Output file path")
                                .takes_value(true)
                                .short('o')
                                .long("output")
                                .required(true),
                        ),
                )
                .subcommand(
                    App::new("delete")
                        .display_order(4)
                        .about("Delete study")
                        .arg(Arg::new("id").about("Study ID").required(true)),
                ),
        )
        .subcommand(
            App::new("series")
                .setting(clap::AppSettings::SubcommandRequiredElseHelp)
                .display_order(2)
                .about("Series-level commands")
                .subcommand(
                    App::new("list")
                        .display_order(0)
                        .about("List all series")
                        .arg(
                            Arg::new("study_id")
                                .takes_value(true)
                                .short('i')
                                .long("study-id")
                                .about("Show only series, belonging to specified study"),
                        ),
                )
                .subcommand(
                    App::new("show")
                        .display_order(1)
                        .about("Show series details")
                        .arg(Arg::new("id").about("Series ID").required(true)),
                )
                .subcommand(
                    App::new("anonymize")
                        .display_order(2)
                        .about("Anonymize series")
                        .arg(Arg::new("id").about("Series ID").required(true))
                        .arg(
                            Arg::new("config")
                                .about("Anonymization configuration file")
                                .takes_value(true)
                                .short('c')
                                .long("config"),
                        ),
                )
                .subcommand(
                    App::new("modify")
                        .display_order(3)
                        .about("Modify series")
                        .arg(Arg::new("id").about("Series ID").required(true))
                        .arg(
                            Arg::new("config")
                                .about("Modification configuration file")
                                .takes_value(true)
                                .short('c')
                                .long("config")
                                .required(true),
                        ),
                )
                .subcommand(
                    App::new("download")
                        .display_order(4)
                        .about("Download series")
                        .arg(Arg::new("id").about("Series ID").required(true))
                        .arg(
                            Arg::new("output")
                                .about("Output file path")
                                .takes_value(true)
                                .short('o')
                                .long("output")
                                .required(true),
                        ),
                )
                .subcommand(
                    App::new("delete")
                        .display_order(4)
                        .about("Delete series")
                        .arg(Arg::new("id").about("Series ID").required(true)),
                ),
        )
        .subcommand(
            App::new("instance")
                .setting(clap::AppSettings::SubcommandRequiredElseHelp)
                .display_order(3)
                .about("Instance-level commands")
                .subcommand(
                    App::new("list")
                        .display_order(0)
                        .about("List all instances")
                        .arg(
                            Arg::new("series_id")
                                .takes_value(true)
                                .short('i')
                                .long("series-id")
                                .about("Show only instances, belonging to specified series"),
                        ),
                )
                .subcommand(
                    App::new("show")
                        .display_order(1)
                        .about("Show instance details")
                        .arg(Arg::new("id").about("Instance ID").required(true)),
                )
                .subcommand(
                    App::new("tags")
                        .display_order(1)
                        .about("Show instance tags")
                        .arg(Arg::new("id").about("Instance ID").required(true)),
                )
                .subcommand(
                    App::new("anonymize")
                        .display_order(2)
                        .about("Anonymize instance")
                        .arg(Arg::new("id").about("Instance ID").required(true))
                        .arg(
                            Arg::new("config")
                                .about("Anonymization configuration file")
                                .takes_value(true)
                                .short('c')
                                .long("config"),
                        )
                        .arg(
                            Arg::new("output")
                                .about("Output file path")
                                .takes_value(true)
                                .short('o')
                                .long("output")
                                .required(true),
                        ),
                )
                .subcommand(
                    App::new("modify")
                        .display_order(3)
                        .about("Modify instance")
                        .arg(Arg::new("id").about("Instance ID").required(true))
                        .arg(
                            Arg::new("config")
                                .about("Modification configuration file")
                                .takes_value(true)
                                .short('c')
                                .long("config")
                                .required(true),
                        )
                        .arg(
                            Arg::new("output")
                                .about("Output file path")
                                .takes_value(true)
                                .short('o')
                                .long("output")
                                .required(true),
                        ),
                )
                .subcommand(
                    App::new("download")
                        .display_order(4)
                        .about("Download instance")
                        .arg(Arg::new("id").about("Instance ID").required(true))
                        .arg(
                            Arg::new("output")
                                .about("Output file path")
                                .takes_value(true)
                                .short('o')
                                .long("output")
                                .required(true),
                        ),
                )
                .subcommand(
                    App::new("delete")
                        .display_order(4)
                        .about("Delete instance")
                        .arg(Arg::new("id").about("Instance ID").required(true)),
                ),
        )
        .subcommand(
            App::new("modality")
                .display_order(3)
                .about("Modality-level commands")
                .subcommand(
                    App::new("list")
                        .display_order(0)
                        .about("List all modalities"),
                )
                .subcommand(
                    App::new("show")
                        .display_order(1)
                        .about("Show modality details")
                        .arg(Arg::new("name").about("Modality name").required(true)),
                )
                .subcommand(
                    App::new("create")
                        .display_order(2)
                        .about("Create a modality")
                        .arg(Arg::new("name").about("Modality name").required(true))
                        .arg(
                            Arg::new("aet")
                                .about("Modality AET")
                                .takes_value(true)
                                .short('a')
                                .long("aet")
                                .required(true),
                        )
                        .arg(
                            Arg::new("host")
                                .about("Modality host")
                                .takes_value(true)
                                .short('h')
                                .long("host")
                                .required(true),
                        )
                        .arg(
                            Arg::new("port")
                                .about("Modality port")
                                .takes_value(true)
                                .short('p')
                                .long("port")
                                .required(true),
                        ),
                )
                .subcommand(
                    App::new("modify")
                        .display_order(2)
                        .about("Modify a modality")
                        .arg(Arg::new("name").about("Modality name").required(true))
                        .arg(
                            Arg::new("aet")
                                .about("Modality AET")
                                .takes_value(true)
                                .short('a')
                                .long("aet")
                                .required(true),
                        )
                        .arg(
                            Arg::new("host")
                                .about("Modality host")
                                .takes_value(true)
                                .short('h')
                                .long("host")
                                .required(true),
                        )
                        .arg(
                            Arg::new("port")
                                .about("Modality port")
                                .takes_value(true)
                                .short('p')
                                .long("port")
                                .required(true),
                        ),
                )
                .subcommand(
                    App::new("echo")
                        .display_order(3)
                        .about("Send a C-ECHO request to a modality")
                        .arg(
                            Arg::new("modality")
                                .about("Modality ID (name)")
                                .required(true),
                        ),
                )
                .subcommand(
                    App::new("store")
                        .display_order(4)
                        .about(
                            "Send entities (patients, studies, series or instances) to a modality",
                        )
                        .arg(
                            Arg::new("modality")
                                .about("Modality ID (name)")
                                .required(true),
                        )
                        .arg(
                            Arg::new("ids")
                                .about("Entity IDs")
                                .takes_value(true)
                                .short('e')
                                .long("entities")
                                .required(true)
                                .multiple_occurrences(true)
                                .multiple_values(true),
                        ),
                )
                .subcommand(
                    App::new("delete")
                        .display_order(5)
                        .about("Delete modality")
                        .arg(Arg::new("name").about("Modality name").required(true)),
                ),
        )
        .get_matches();

    let mut server_address = "".to_string();
    match get_server_address(orthanc.value_of("server")) {
        Ok(s) => server_address = s,
        Err(e) => exit_with_error(e),
    };
    let o = match Orthanc::new(
        server_address,
        get_username(orthanc.value_of("username")),
        get_password(orthanc.value_of("password")),
    ) {
        Ok(o) => o,
        Err(e) => return exit_with_error(e),
    };

    match orthanc.subcommand() {
        Some(("patient", patient)) => match patient.subcommand() {
            Some(("list", _)) => match o.list_patients() {
                Ok(t) => print(t),
                Err(e) => exit_with_error(e),
            },
            Some(("show", show)) => match o.show_patient(show.value_of("id").unwrap()) {
                Ok(t) => print(t),
                Err(e) => exit_with_error(e),
            },
            Some(("anonymize", anonymize)) => match o.anonymize_patient(
                anonymize.value_of("id").unwrap(),
                anonymize.value_of("config"),
            ) {
                Ok(t) => print(t),
                Err(e) => exit_with_error(e),
            },
            Some(("modify", modify)) => match o.modify_patient(
                modify.value_of("id").unwrap(),
                modify.value_of("config").unwrap(),
            ) {
                Ok(t) => print(t),
                Err(e) => exit_with_error(e),
            },
            Some(("download", download)) => match o.download_patient(
                download.value_of("id").unwrap(),
                download.value_of("output").unwrap(),
            ) {
                Ok(_) => (),
                Err(e) => exit_with_error(e),
            },
            Some(("delete", delete)) => match o.delete_patient(delete.value_of("id").unwrap()) {
                Ok(_) => (),
                Err(e) => exit_with_error(e),
            },
            _ => {}
        },
        Some(("study", study)) => match study.subcommand() {
            Some(("list", list)) => match o.list_studies(list.value_of("patient_id")) {
                Ok(t) => print(t),
                Err(e) => exit_with_error(e),
            },
            Some(("show", show)) => match o.show_study(show.value_of("id").unwrap()) {
                Ok(t) => print(t),
                Err(e) => exit_with_error(e),
            },
            Some(("anonymize", anonymize)) => match o.anonymize_study(
                anonymize.value_of("id").unwrap(),
                anonymize.value_of("config"),
            ) {
                Ok(t) => print(t),
                Err(e) => exit_with_error(e),
            },
            Some(("modify", modify)) => match o.modify_study(
                modify.value_of("id").unwrap(),
                modify.value_of("config").unwrap(),
            ) {
                Ok(t) => print(t),
                Err(e) => exit_with_error(e),
            },
            Some(("download", download)) => match o.download_study(
                download.value_of("id").unwrap(),
                download.value_of("output").unwrap(),
            ) {
                Ok(_) => (),
                Err(e) => exit_with_error(e),
            },
            Some(("delete", delete)) => match o.delete_study(delete.value_of("id").unwrap()) {
                Ok(_) => (),
                Err(e) => exit_with_error(e),
            },
            _ => {}
        },
        Some(("series", series)) => match series.subcommand() {
            Some(("list", list)) => match o.list_series(list.value_of("study_id")) {
                Ok(t) => print(t),
                Err(e) => exit_with_error(e),
            },
            Some(("show", show)) => match o.show_series(show.value_of("id").unwrap()) {
                Ok(t) => print(t),
                Err(e) => exit_with_error(e),
            },
            Some(("anonymize", anonymize)) => match o.anonymize_series(
                anonymize.value_of("id").unwrap(),
                anonymize.value_of("config"),
            ) {
                Ok(t) => print(t),
                Err(e) => exit_with_error(e),
            },
            Some(("modify", modify)) => match o.modify_series(
                modify.value_of("id").unwrap(),
                modify.value_of("config").unwrap(),
            ) {
                Ok(t) => print(t),
                Err(e) => exit_with_error(e),
            },
            Some(("download", download)) => match o.download_series(
                download.value_of("id").unwrap(),
                download.value_of("output").unwrap(),
            ) {
                Ok(_) => (),
                Err(e) => exit_with_error(e),
            },
            Some(("delete", delete)) => match o.delete_series(delete.value_of("id").unwrap()) {
                Ok(_) => (),
                Err(e) => exit_with_error(e),
            },
            _ => {}
        },
        Some(("instance", instance)) => match instance.subcommand() {
            Some(("list", list)) => match o.list_instances(list.value_of("series_id")) {
                Ok(t) => print(t),
                Err(e) => exit_with_error(e),
            },
            Some(("show", show)) => match o.show_instance(show.value_of("id").unwrap()) {
                Ok(t) => print(t),
                Err(e) => exit_with_error(e),
            },
            Some(("anonymize", anonymize)) => match o.anonymize_instance(
                anonymize.value_of("id").unwrap(),
                anonymize.value_of("config"),
                anonymize.value_of("output").unwrap(),
            ) {
                Ok(_) => (),
                Err(e) => exit_with_error(e),
            },
            Some(("modify", modify)) => match o.modify_instance(
                modify.value_of("id").unwrap(),
                modify.value_of("config").unwrap(),
                modify.value_of("output").unwrap(),
            ) {
                Ok(_) => (),
                Err(e) => exit_with_error(e),
            },
            Some(("tags", tags)) => match o.show_instance_tags(tags.value_of("id").unwrap()) {
                Ok(t) => print(t),
                Err(e) => exit_with_error(e),
            },
            Some(("download", download)) => match o.download_instance(
                download.value_of("id").unwrap(),
                download.value_of("output").unwrap(),
            ) {
                Ok(_) => (),
                Err(e) => exit_with_error(e),
            },
            Some(("delete", delete)) => match o.delete_instance(delete.value_of("id").unwrap()) {
                Ok(_) => (),
                Err(e) => exit_with_error(e),
            },
            _ => {}
        },
        Some(("modality", modality)) => match modality.subcommand() {
            Some(("list", _)) => match o.list_modalities() {
                Ok(t) => print(t),
                Err(e) => exit_with_error(e),
            },
            Some(("show", show)) => match o.show_modality(show.value_of("name").unwrap()) {
                Ok(t) => print(t),
                Err(e) => exit_with_error(e),
            },
            Some(("create", create)) => match o.create_modality(
                create.value_of("name").unwrap(),
                create.value_of("aet").unwrap(),
                create.value_of("host").unwrap(),
                create.value_of("port").unwrap().parse::<i32>().unwrap(),
            ) {
                Ok(_) => (),
                Err(e) => exit_with_error(e),
            },
            Some(("modify", modify)) => match o.modify_modality(
                modify.value_of("name").unwrap(),
                modify.value_of("aet").unwrap(),
                modify.value_of("host").unwrap(),
                modify.value_of("port").unwrap().parse::<i32>().unwrap(),
            ) {
                Ok(_) => (),
                Err(e) => exit_with_error(e),
            },
            Some(("store", store)) => {
                let ids: Vec<&str> = store.values_of("ids").unwrap().collect();
                match o.do_store(store.value_of("modality").unwrap(), &ids) {
                    Ok(t) => print(t),
                    Err(e) => exit_with_error(e),
                }
            }
            Some(("delete", delete)) => match o.delete_modality(delete.value_of("name").unwrap()) {
                Ok(_) => (),
                Err(e) => exit_with_error(e),
            },
            _ => {}
        },
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::{remove_var, set_var};

    #[test]
    fn test_get_server() {
        remove_var("ORC_ORTHANC_ADDRESS");
        assert_eq!(get_server_address(Some("foo")).unwrap(), "foo".to_string());
        assert_eq!(
            get_server_address(None).unwrap_err(),
            CliError::new(
                "Command error",
                Some("Neither --server-address nor ORC_ORTHANC_ADDRESS are set"),
                Some("environment variable not found"),
            )
        );
        set_var("ORC_ORTHANC_ADDRESS", "bar");
        assert_eq!(get_server_address(None).unwrap(), "bar".to_string());
        assert_eq!(get_server_address(Some("baz")).unwrap(), "baz".to_string())
    }

    #[test]
    fn test_get_username() {
        remove_var("ORC_ORTHANC_USERNAME");
        assert_eq!(get_username(Some("foo")).unwrap(), "foo".to_string());
        assert_eq!(get_username(None), None);
        set_var("ORC_ORTHANC_USERNAME", "bar");
        assert_eq!(get_username(Some("foo")).unwrap(), "foo".to_string());
        assert_eq!(get_username(None).unwrap(), "bar".to_string());
    }

    #[test]
    fn test_get_password() {
        remove_var("ORC_ORTHANC_PASSWORD");
        assert_eq!(get_password(Some("foo")).unwrap(), "foo".to_string());
        assert_eq!(get_password(None), None);
        set_var("ORC_ORTHANC_PASSWORD", "bar");
        assert_eq!(get_password(Some("foo")).unwrap(), "foo".to_string());
        assert_eq!(get_password(None).unwrap(), "bar".to_string());
    }
}
