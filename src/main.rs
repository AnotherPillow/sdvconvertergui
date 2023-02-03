//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::fs;
use std::io;
use std::env;
use std::io::Write;
use std::process::Command;
use std::path::Path;
use eframe::egui;
use reqwest::blocking::Client;
use tracing_subscriber;
use serde_json::json;
//use zip::read::ZipArchive;
use zip::ZipArchive;
//use rfd::FileDialog;
// let _files = FileDialog::new()
//         .add_filter("text", &["txt", "rs"])
//         .add_filter("rust", &["rs", "toml"])
//         .set_directory("/")
//         .pick_file();
//use std::error::Error;

fn main()  {
    if std::path::Path::new("converters/").exists() {
        fs::remove_dir_all("converters/").expect("Unable to delete converters folder");
    }
    std::fs::create_dir("converters/").expect("Failed to create converters folder");

    

    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(640.0, 480.0)),
        ..Default::default()
    };
    eframe::run_native(
        "SDV Converter GUI",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    mod_type: String,
    manifest_notif: bool,
    manifest_path: String,
    input_folder: String,
    run_converter: bool,
    compat_notif: bool,
    output_data: String,
    check_py: bool,
    py_cmd: String,
    converter_complete: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            mod_type: "Please select a type of mod to convert!".to_string(),
            manifest_notif: false,
            manifest_path: "".to_string(),
            input_folder: "".to_string(),
            run_converter: false,
            compat_notif: false,
            output_data: "Run a converter to see it's output!".to_string(),
            check_py: true,
            py_cmd: "".to_string(),
            converter_complete:false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut converters = json!({
            "TMXL2CP":{
                "name":"TMXL2CP",
                "description":"Convert TMXLoader mods to Content Patcher",
                "input_folder":"TMXL/",
                "input_mode": "plain",
                "output_folder":"CP/",
                "output_mode": "plain",
                "main_file":"main.py",
                "windows_dependencies":"py -m pip install -r requirements.txt",
                "linux_dependencies":"python3 -m pip install -r requirements.txt",
                "url":"https://github.com/anotherpillow/TMXL2CP",
                "zip_link":"https://github.com/AnotherPillow/TMXL2CP/archive/refs/heads/main.zip",
                "author":"AnotherPillow",
                "unique_id_support": "Platonymous.TMXLoader",
                "branch_file_name":"TMXL2CP-main",
            },
            "CP2AT":{
                "name":"CP2AT",
                "description":"Convert Content Patcher mods to Alternate Textures",
                "input_folder":"[CP] Mod to Convert",
                "input_mode": "modname",
                "output_folder":"[AT] Mod to Convert",
                "output_mode": "modname",
                "main_file":"main.py",
                "windows_dependencies":"py -m pip install -r requirements.txt",
                "linux_dependencies":"python3 -m pip install -r requirements.txt",
                "url":"https://github.com/holy-the-sea/CP2AT",
                "unique_id_support": "Pathoschild.ContentPatcher",
                "branch_file_name":"CP2AT-main",
                "zip_link":"https://github.com/holy-the-sea/CP2AT/archive/refs/heads/main.zip",
                "author":"holythesea",
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("SDV Converter GUI");
            ui.separator();

            if self.check_py {
                let py_versions = ["Python 3.8", "Python 3.9", "Python 3.10"];
                let operating_sys = env::consts::OS;
                if operating_sys == "windows" {
                    let output = Command::new("py")
                        .arg("--version")
                        .output()
                        .expect("failed to execute process");
                    let output = String::from_utf8_lossy(&output.stdout);
                    self.py_cmd = "py".to_string();
                    println!("Python installed version: {}", output);
                    if !py_versions.iter().any(|&x| output.contains(x)) {
                        ui.label("Python 3.8, 3.9, or 3.10 is not installed!");
                        ui.label("Please install Python 3.8, 3.9, or 3.10 to use this program! https://www.python.org/downloads/");
                        if ui.button("Ok").clicked() {
                            self.check_py = false;
                        }
                    } else {
                        self.check_py = false;
                    }
                    
                } else {
                    let output = Command::new("python3")
                        .arg("--version")
                        .output()
                        .expect("failed to execute process");
                    let output = String::from_utf8_lossy(&output.stdout);
                    self.py_cmd = "python3".to_string();
                    if !py_versions.iter().any(|&x| x.contains(&*output)) {
                        ui.label("Python 3.8, 3.9, or 3.10 is not installed!");
                        ui.label("Please install Python 3.8, 3.9, or 3.10 to use this program! https://www.python.org/downloads/");
                        if ui.button("Ok").clicked() {
                            self.check_py = false;
                        }
                    } else {
                        self.check_py = false;
                    }
                }
            }
            //have a box to select a folder
            ui.label("Select a folder to convert");
            


            egui::ComboBox::from_id_source("mod_type")
                .selected_text(&self.mod_type)
                .width(200.0)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.mod_type, "TMXL2CP".to_string(), "TMXLoader [To Content Patcher] by AnotherPillow");
                    ui.selectable_value(&mut self.mod_type, "CP2AT".to_string(), "Content Patcher [To Alternate Textures] by holythesea");
                });
            //add a button to choose mod

            if self.compat_notif {
                ui.label("This mod is not compatible with this converter!");
                if ui.button("Ok").clicked() {
                    self.compat_notif = false;
                }
            }

            if self.run_converter {
                let converter_info = &mut converters[&self.mod_type];
                println!("Mod type: {}", self.mod_type);
                println!("Converter: {}", converter_info["name"]);
                println!("Input folder: {}", self.input_folder);
                println!("Manifest path: {}", self.manifest_path);
                let input_dir = self.input_folder.clone();
                let input_file_parent_folder_name = Path::new(&input_dir).parent().unwrap().file_name().unwrap().to_str().unwrap();

                if converter_info["name"] == "CP2AT" {
                    converter_info["input_folder"] = serde_json::Value::String(input_file_parent_folder_name.to_string());
                    converter_info["output_folder"] = serde_json::Value::String(input_file_parent_folder_name.to_string().replace("[CP]", "[AT]"));
                }

                //load manifest.json
                let manifest_raw = fs::File::open(self.manifest_path.clone()).expect("file should open read only");
                let manifest_json: serde_json::Value = serde_json::from_reader(manifest_raw).expect("file should be proper JSON");
                println!("Manifest: {:#}", manifest_json);

                if manifest_json["ContentPackFor"]["UniqueID"] != converter_info["unique_id_support"] {
                    self.compat_notif = true;
                    self.run_converter = false;
                    return;
                }

                //download the converter
                let zip_path_str = format!("converters/{}.zip", converter_info["name"].as_str().unwrap());
                println!("Zip path: {}", zip_path_str);
                let zip_path = std::path::Path::new(zip_path_str.as_str());
                let mut zipfile_dest = fs::File::create(zip_path).unwrap();
                let zipfile_client = Client::new();
                let mut zipfile_response = zipfile_client.get(converter_info["zip_link"].as_str().unwrap()).send().unwrap();
                io::copy(&mut zipfile_response, &mut zipfile_dest).unwrap();
                println!("Downloaded zip file");

                let mut zip_archive = ZipArchive::new(fs::File::open(zip_path).unwrap()).unwrap();
                for i in 0..zip_archive.len() {
                    let mut file = zip_archive.by_index(i).unwrap();
                    
                    let outpath = Path::new("converters").join(file.sanitized_name());
                    if (&*file.name()).ends_with('/') {
                        println!("File {} extracted to \"{}\"", i, outpath.display());
                        fs::create_dir_all(&outpath).unwrap();
                    } else {
                        println!("File {} extracted to \"{}\" ({} bytes)", i, outpath.display(), file.size());
                        if let Some(p) = outpath.parent() {
                            if !p.exists() {
                                fs::create_dir_all(&p).unwrap();
                            }
                        }
                        let mut outfile = fs::File::create(&outpath).unwrap();
                        io::copy(&mut file, &mut outfile).unwrap();
                    }
                }
                
                //copy everything from the input dir to the converter's input dir
                
                let converter_input_dir = Path::new(&env::current_dir().unwrap()).join("converters").join(converter_info["branch_file_name"].as_str().unwrap()).join(converter_info["input_folder"].as_str().unwrap());
                let converter_input_dir_path = converter_input_dir.to_str().unwrap();
                let converter_dir = Path::new("converters").join(converter_info["branch_file_name"].as_str().unwrap());
                
                if !converter_input_dir.exists() {
                    fs::create_dir_all(&converter_input_dir).unwrap();
                } else {
                    fs::remove_dir_all(&converter_input_dir).unwrap();
                    fs::create_dir_all(&converter_input_dir).unwrap();
                }

                if converter_info["name"].as_str().unwrap() == "CP2AT" {
                    //open config.json file of converter
                    let newconfig = json!({
                        "mod_folder_path": converter_input_dir.clone(),
                        "keywords": [],
                    }).to_string();
                    let mut config_file = fs::File::create(converter_dir.join("config.json")).unwrap();
                    config_file.write_all(newconfig.as_bytes()).unwrap();
                }

                
                Command::new(self.py_cmd.clone())
                    .arg("-m")
                    .arg("pip")
                    .arg("install")
                    .arg("-r")
                    .arg("requirements.txt")
                    .current_dir(converter_dir.clone())
                    .output()
                    .expect("failed to execute process");

                println!("input dir: {} converter input dir: {}", input_dir, converter_input_dir_path);
                    
                copy_folder(&input_dir, converter_input_dir_path);
                
                let conversion = Command::new(self.py_cmd.clone())
                    .arg(converter_info["main_file"].as_str().unwrap())
                    .current_dir(converter_dir.clone())
                    .output()
                    .expect("failed to execute process");
                //println!("conv {:?}", conversion.status);
                let mut convert_output = String::from_utf8_lossy(&conversion.stdout).to_string();
                //println!("Conversion output: {}", convert_output);
                let cd_result = &env::current_dir().unwrap();
                let outdir_fullpath = cd_result.join(converter_dir.join(converter_info["output_folder"].as_str().unwrap().replace("/", "\\")));
                convert_output += "\n";
                convert_output += format!("You can find your converted mod in the \"{}\" folder", outdir_fullpath.display()).as_str();
                self.output_data = convert_output;
                
                
                
                self.converter_complete = true;

                



                
                self.run_converter = false;
            }

            ui.separator();
            if ui.button("Select mod").clicked() && self.manifest_notif == false && self.compat_notif == false {
                let file = rfd::FileDialog::new()
                    .add_filter("manifest.json", &["json"])
                    .set_directory("/")
                    .pick_file();
                //exit the if it is a content pack
                
                let path = file.unwrap();
                if !path.to_str().unwrap().ends_with("manifest.json") {
                    self.manifest_notif = true;
                    return;
                }
                //println!("Path: {}", path.to_str().unwrap());
                //find the directory of the file
                let dir = path.parent().unwrap();
                //println!("Dir: {}", dir.to_str().unwrap());
                self.manifest_path = path.to_str().unwrap().to_string();
                self.input_folder = dir.to_str().unwrap().to_string();
                if self.mod_type != "Please select a type of mod to convert!".to_string() {
                    self.run_converter = true;
                }
            }
            if self.manifest_notif {
                ui.label("You didn't select a manifest.json file, please select a manifest.json file.");
                if ui.button("Ok").clicked() {
                    self.manifest_notif = false;
                }
            }
        
            //have a textarea on the right to show the output

            if self.converter_complete {
                ui.label("Conversion complete!");
                if ui.button("Ok").clicked() {
                    self.converter_complete = false;
                }
            }

            ui.separator();
            ui.label("Output");
            
            
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add(egui::TextEdit::multiline(&mut self.output_data.clone()).desired_rows(10));
            });




                
            
            ui.separator();
            
        });
    }
}

fn copy_folder(src: &str, dest: &str) {
    let src = Path::new(src);
    let dest = Path::new(dest);

    for entry in fs::read_dir(src).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        println!("Copying: {}", path.display());

        let file_name = path.file_name().unwrap();
        let new_path = dest.join(file_name);
        if path.is_file() {
            //make folders needed for new path if they don't exist
            if let Some(p) = new_path.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            fs::copy(path, new_path).unwrap();
        } else if path.is_dir() {
            copy_folder(path.to_str().unwrap(), dest.join(file_name).to_str().unwrap());
        }
    }
}