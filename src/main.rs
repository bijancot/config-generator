use std::{fs, env};
use tera::{Tera, Context};

fn main() {
    let config_file = "/template/hello-world.txt.template";
    let curr_dir = env::current_dir().unwrap().to_str().unwrap().to_string();
    let config_dir = curr_dir.to_string() + config_file;
    
    let mut tera = Tera::default();

    let data = fs::read_to_string(config_dir).expect("Unable to read file");

    tera.add_raw_template("panji-template", &data).expect("something wrong from template");
    
    // let text = format!(r#"{}"#, data);
    let mut context = Context::new();


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
    
    context.insert("name", "panji");
    context.insert("workplace", "detikcom");
    context.insert("website", "https://panjibaskoro.web.id");
    context.insert("language", "rust");

    let resu = tera.render("panji-template", &context);

    fs::write(curr_dir+"/panji.txt", resu.unwrap().as_str()).expect("something wrong");
}
