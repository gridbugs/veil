#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate toml;
extern crate handlebars;
extern crate image;

use std::io::Read;
use std::io::Write;
use std::fs::{self, File};
use std::path::Path;
use std::collections::{HashMap, HashSet};

use handlebars::Handlebars;
use image::{FilterType, GenericImage, ColorType};

const COMPONENT_PATH: &'static str = "components.toml";

fn ret_none() -> Option<String> { None }

#[derive(Debug, Deserialize)]
struct SpatialHashDesc {
    imports: HashSet<String>,
    position_component: String,
    fields: HashMap<String, SpatialHashFieldDesc>,
}

#[derive(Debug, Deserialize)]
struct SpatialHashFieldDesc {
    component: String,
    aggregate: String,
}

#[derive(Debug, Serialize)]
struct SpatialHashDescOut {
    imports: HashSet<String>,
    position_component: String,
    position_type: String,
    components: HashMap<String, SpatialHashComponentDescOut>,
}

#[derive(Debug, Serialize)]
struct SpatialHashComponentDescOut {
    #[serde(rename = "type", default = "ret_none")]
    type_name: Option<String>,
    fields: HashMap<String, SpatialHashFieldDescOut>,
}

#[derive(Debug, Serialize)]
struct SpatialHashFieldDescOut {
    aggregate_name: String,
    aggregate_type: String,
    aggregate_cons: String,
    void: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct ComponentDesc {
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

fn read_spatial_hash_desc<P: AsRef<Path>>(path: P) -> SpatialHashDesc {
    toml::from_str(&read_file(path)).expect("Failed to parse spatial hash desc")
}

fn make_handlebars() -> Handlebars {
    let mut handlebars = Handlebars::new();
    // prevent xml escaping
    handlebars.register_escape_fn(|input| input.to_string());
    handlebars
}

fn render_entity_system_template_internal<P: AsRef<Path>>(desc: EntityStoreDesc,
                                   template_path: P) -> String {

    let template = read_file(template_path);

    let EntityStoreDesc { imports, components } = desc;

    let entity_store_desc_out = EntityStoreDescOut {
        imports: imports,
        components: components,
    };

    make_handlebars().template_render(template.as_ref(), &entity_store_desc_out)
        .expect("Failed to render template")
}

fn render_spatial_hash_template_internal<P: AsRef<Path>>(desc: SpatialHashDesc,
                                                         type_desc: EntityStoreDesc,
                                                         template_path: P) -> String {

    let template = read_file(template_path);

    let SpatialHashDesc { imports, position_component, fields } = desc;
    let EntityStoreDesc { components, .. } = type_desc;

    let mut components_out = HashMap::new();

    for (field_name, field) in fields.iter() {
        let component_desc = components.get(&field.component).expect(&format!("No such component: {}", field_name));

        let (aggregate_type, aggregate_cons) = match field.aggregate.as_ref() {
            "count" => ("usize", "0"),
            "f64_total" => ("f64", "0.0"),
            "set" => ("HashSet<EntityId>", "HashSet::new()"),
            "void" => ("", ""),
            other => panic!("No such aggregate: {}", other),
        };

        let mut component = components_out.entry(field.component.clone()).or_insert_with(|| SpatialHashComponentDescOut {
            type_name: component_desc.type_name.clone(),
            fields: HashMap::new(),
        });

        let field_out = SpatialHashFieldDescOut {
            aggregate_name: field_name.clone(),
            aggregate_type: aggregate_type.to_string(),
            aggregate_cons: aggregate_cons.to_string(),
            void: field.aggregate == "void",
        };

        component.fields.insert(field.aggregate.clone(), field_out);
    }

    let desc_out = SpatialHashDescOut {
        imports: imports,
        position_component: position_component.clone(),
        position_type: components.get(&position_component)
            .expect(&format!("No such component: {}", &position_component))
            .type_name.clone().expect("Position component must have associated data"),
        components: components_out,
    };

    make_handlebars().template_render(template.as_ref(), &desc_out)
        .expect("Failed to render template")
}


fn source_changed_rel<P: AsRef<Path>, Q: AsRef<Path>>(in_path: P, out_path: Q) -> bool {
    if !out_path.as_ref().exists() {
        return true;
    }
    let out_time = if let Ok(md) = fs::metadata(out_path) {
        md.modified().expect("Failed to get output file modified time")
    } else {
        return true;
    };

    let in_time = fs::metadata(in_path).expect("Missing input file")
        .modified().expect("Failed to get input file modified time");

    in_time > out_time
}

fn render_entity_system_template() {
    let out_path = "src/entity_store/generated_component_list_macros.rs";
    let template_path = "src/entity_store/template.rs.hbs";

    if source_changed_rel(COMPONENT_PATH, out_path) || source_changed_rel(template_path, out_path) {
        let type_desc = read_entity_store_desc(COMPONENT_PATH);
        let output = render_entity_system_template_internal(type_desc, template_path);
        write_file(out_path, output);
    }
}

fn render_spatial_hash_template() {
    let out_path = "src/spatial_hash/generated_component_list_macros.rs";
    let in_path = "spatial_hash.toml";
    let template_path = "src/spatial_hash/template.rs.hbs";

    if source_changed_rel(in_path, out_path) || source_changed_rel(template_path, out_path) {
        let desc = read_spatial_hash_desc(in_path);
        let type_desc = read_entity_store_desc(COMPONENT_PATH);
        let output = render_spatial_hash_template_internal(desc, type_desc, template_path);
        write_file(out_path, output);
    }
}

fn scale_tiles() {

    const IN_PATH: &'static str = "resources/tiles.png";
    const OUT_PATH: &'static str = "resources/tiles_scaled.png";
    const TILE_SCALE: u32 = 2;

    if source_changed_rel(IN_PATH, OUT_PATH) {

        let original = image::open(IN_PATH).expect(format!("Failed to open image: {}", IN_PATH).as_ref());

        let (width, height) = original.dimensions();
        let scaled = original.resize_exact(width * TILE_SCALE, height * TILE_SCALE, FilterType::Nearest).to_rgba();

        let (width, height) = scaled.dimensions();
        image::save_buffer(OUT_PATH, &scaled, width, height, ColorType::RGBA(8))
            .expect(format!("Failed to save scaled image: {}", OUT_PATH).as_ref());
    }

}

fn main() {
    render_entity_system_template();
    render_spatial_hash_template();
    scale_tiles();
}
