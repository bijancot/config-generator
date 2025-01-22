extern crate tera;
extern crate serde_json;


use std::{fs, env};
use tera::{Tera, Context};

use serde_json::Value;

fn main() {
    let config_file = "/template/nginx-minimal.template";
    let json_file = "/template/sample.json";
    let curr_dir = env::current_dir().unwrap().to_str().unwrap().to_string();
    let config_dir = curr_dir.to_string() + config_file;
    let json_dir = curr_dir.to_string() + json_file;
    
    let mut tera = Tera::default();

    let data = fs::read_to_string(config_dir).expect("Unable to read file");
    let json_data = fs::read_to_string(json_dir).expect("json issues");

    tera.add_raw_template("panji-template", &data).expect("something wrong from template");
    
    // let text = format!(r#"{}"#, data);
    let mut context = Context::new();


    let test_json = json_data.as_str();

    // println!("{}",test_json.to_string());
    let v: Value = serde_json::from_str(test_json).expect("hift");

    // let text = r##"
    //     panji
    //     gateng 
    //                 sdasd
    //                 dadas
    //             asdsad
    //     dasd
    //         dsd
    //         - dsd
    //         - sds 
    //         - sdsd
    // "##;
    // println!("Please call {} at the number {}", v["project_name"], v["domain_name"]);

    let project_name = &v["project_name"];
    let dir = &v["directory"];
    let domain = &v["domain_name"];

    context.insert("project_name", project_name);
    context.insert("directory", dir);
    context.insert("domain_name", domain);

    // println!("{:?}", context);
    let resu = tera.render("panji-template", &context);
    // println!("{:?}", resu.unwrap());
    fs::write(curr_dir+"/"+project_name.as_str().unwrap()+".txt", resu.unwrap().as_str()).expect("something wrong");

    // println!("{}",curr_dir);
}
