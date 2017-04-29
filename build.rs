#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate toml;
extern crate handlebars;

use std::io::Read;
use std::io::Write;
use std::fs::{self, File};
use std::path::Path;
use std::collections::HashMap;

use handlebars::Handlebars;

fn ret_none() -> Option<String> { None }

#[derive(Debug, Serialize, Deserialize)]
struct ComponentDesc {
    name: String,
    #[serde(rename = "type", default = "ret_none")]
    type_name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct EntityStoreDesc {
    imports: Vec<String>,
    components: HashMap<String, ComponentDesc>,
}

#[derive(Debug, Serialize)]
struct EntityStoreDescOut {
    imports: Vec<String>,
    components: HashMap<String, ComponentDesc>,
    num_components: usize,
    component_bits: usize,
}

fn read_file<P: AsRef<Path>>(path: P) -> String {
    let mut data = String::new();
    let mut f = File::open(path).expect("Unable to open file");
    f.read_to_string(&mut data).expect("Unable to read file");
    data
}

fn write_file<P: AsRef<Path>>(path: P, s: String) {
    let mut f = File::create(path).expect("Unable to create file");
    f.write_all(s.as_bytes()).expect("Unable to write file");
}

fn read_entity_store_desc<P: AsRef<Path>>(path: P) -> EntityStoreDesc {
    toml::from_str(&read_file(path)).expect("Failed to parse entity store desc")
}

fn render_template<P: AsRef<Path>>(desc: EntityStoreDesc,
                                   template_path: P) -> String {

    let template = read_file(template_path);

    let mut handlebars = Handlebars::new();
    // prevent xml escaping
    handlebars.register_escape_fn(|input| input.to_string());

    let EntityStoreDesc { imports, components } = desc;

    let num_components = components.len() as u64;
    let component_bits = 64 - (num_components - 1).leading_zeros();

    let entity_store_desc_out = EntityStoreDescOut {
        num_components: num_components as usize,
        component_bits: component_bits as usize,
        imports: imports,
        components: components,
    };

    handlebars.template_render(template.as_ref(), &entity_store_desc_out)
        .expect("Failed to render template")
}

fn source_changed_rel<P: AsRef<Path>, Q: AsRef<Path>>(in_path: P, out_path: Q) -> bool {
    let out_time = if let Ok(md) = fs::metadata(out_path) {
        md.modified().expect("Failed to get output file modified time")
    } else {
        return true;
    };

    let in_time = fs::metadata(in_path).expect("Missing input file")
        .modified().expect("Failed to get input file modified time");

    in_time > out_time
}

fn main() {
    let out_path = "src/entity_store/generated_component_list_macros.rs";
    let in_path = "types.toml";
    let template_path = "src/entity_store/template.rs.hbs";

    if source_changed_rel(in_path, out_path) || source_changed_rel(template_path, out_path) {
        let type_desc = read_entity_store_desc(in_path);
        let output = render_template(type_desc, template_path);
        write_file(out_path, output);
    }
}
