use std::{println, path::PathBuf, ffi::OsString};
use ini::Ini;
use procfs::process::all_processes;

pub struct OpenedProfile {
    pub name: String,
    pub pid: usize,

}

fn main() {

    let opened = get_openned_profiles();
    let profiles_path = get_profiles_path().unwrap();
    let conf = Ini::load_from_file(profiles_path).unwrap();
    
    for (sec, props) in conf.iter() {
        let section = sec.as_ref().unwrap_or(&"");
        if section.starts_with("Profile") {
            let name = props.get("Name");
            if ! opened.contains(&name.unwrap().to_string()) {
                println!("{}", name.unwrap())
            }
        }

    }
}

fn get_profiles_path() -> Option<PathBuf> {
    match home::home_dir() {
        Some(path) => Some(path.join(".mozilla/firefox/profiles.ini").to_owned()),
        None => None,
    }
}

fn get_openned_profiles() -> Vec<String> {
    let mut profiles = all_processes().unwrap().filter(|process| {
        match process.as_ref() {
            Ok(proc) => match proc.environ() {
                Ok(environ) => match environ.get(&OsString::from("MANAGED_PROFILE")) {
                    Some(_) => true,
                    None => false
                },
                Err(_) => false,
            },
            Err(_) => false
        }
    }).map(|process| process
            .as_ref().unwrap()
            .environ().unwrap()
            .get(&OsString::from("MANAGED_PROFILE")).unwrap()
            .clone().into_string().unwrap()
        ).collect::<Vec<_>>();
    profiles.dedup();
    profiles
}
