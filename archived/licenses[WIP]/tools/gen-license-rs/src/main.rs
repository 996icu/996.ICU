use clap::*;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;

/*

usage: 

[generate a mit LICENSE file in current directory]
gen-license-rs mit  

[list license types]
gen-license-rs --list

[generate license with 996ICU]
gen-license-rs mit --996icu en-us
gen-license-rs mit --996icu zh-cn

[show help]
gen-license-rs -h

[show version]
gen-license-rs -V

*/
fn main() {
     let all_licenses: HashMap<&'static str, &'static str> = {
          let mut ans = HashMap::new();
          ans.insert("agpl-3.0", include_str!("licenses/agpl-3.0.txt"));
          ans.insert("apache-2.0", include_str!("licenses/apache-2.0.txt"));
          ans.insert("bsd-2-clause", include_str!("licenses/bsd-2-clause.txt"));
          ans.insert("bsd-3-clause", include_str!("licenses/bsd-3-clause.txt"));
          ans.insert("epl-2.0", include_str!("licenses/epl-2.0.txt"));
          ans.insert("gpl-2.0", include_str!("licenses/gpl-2.0.txt"));
          ans.insert("gpl-3.0", include_str!("licenses/gpl-3.0.txt"));
          ans.insert("lgpl-2.1", include_str!("licenses/lgpl-2.1.txt"));
          ans.insert("lgpl-3.0", include_str!("licenses/lgpl-3.0.txt"));
          ans.insert("mit", include_str!("licenses/mit.txt"));
          ans.insert("mpl-2.0", include_str!("licenses/mpl-2.0.txt"));
          ans.insert("unlicenses", include_str!("licenses/unlicenses.txt"));
          ans.insert("996icu-0.1", include_str!("licenses/996icu-0.1.txt"));
          ans
     };
     let all_icus: HashMap<&'static str, &'static str> = {
          let mut ans = HashMap::new();
          ans.insert("en-us", include_str!("licenses/996.icu.template.en-us.txt"));
          ans.insert("zh-cn", include_str!("licenses/996.icu.template.zh-cn.txt"));
          ans
     }; 
     let app = App::new(crate_name!())
          .version(crate_version!())
          .author(crate_authors!())
          .about(crate_description!())
          .arg(Arg::with_name("license-type")
               .value_name("LICENSE TYPE")
               .takes_value(true)
               .help("the license type you want to generate"))
          .arg(Arg::with_name("list")
               .short("l")
               .long("list")
               .help("show supported license types"))
          .arg(Arg::with_name("996icu")
               .long("996icu")
               .takes_value(true)
               .value_name("LANGUAGE")
               .help("expand license with 996ICU license, choose a language parameter or default zh-cn"))
          ;
     let mut write = vec![0u8; 4096];
     app.write_help(&mut write).expect("write help message to buffer");
     let help_msg = String::from_utf8_lossy(&write);
     let matches = app.get_matches();
     if matches.is_present("list") {
          for license in all_licenses.keys() {
               println!("{}", license);
          }
          return;
     }
     let license_type = if let Some(t) = matches.value_of("license-type") { t } 
     else {
          println!("{}", help_msg);
          return;
     };
     if let Some(&content) = all_licenses.get(&license_type) {
          let mut file = OpenOptions::new()
               .write(true)
               .create(true)
               .open("LICENSE")
               .expect("open or create LICENSE file");
          let output = if let Some(icu) = matches.value_of("996icu") {
               let template = match icu.trim().to_lowercase().as_str() {
                    "zh" | "zh-cn" | "zh-hans" => all_icus.get("zh-cn").unwrap(),
                    "" | "en" | "en-us" => all_icus.get("en-us").unwrap(),
                    _ => {
                         println!("error: invalid language choice '{}'", icu);
                         print!("note: choose one from (");
                         for license in all_icus.keys() {
                              print!("'{}', ", license)
                         }
                         println!(")");
                         println!("help: use '-h' for more information");
                         return;
                    }
               };
               template
                    .replace("{other}", license_type)
                    .replace("{content}", content)
          } else {
               String::from(content)
          };
          file.write_fmt(format_args!("{}", output))
               .expect("write to LICENSE file");
     } else {
          println!("error: invalid license choice '{}'", license_type);
          print!("note: choose one from (");
          for license in all_licenses.keys() {
               print!("'{}', ", license)
          }
          println!(")");
          println!("help: use '-h' for more information");
     }
}
