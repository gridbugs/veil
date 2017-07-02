#![allow(dead_code)]

#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate toml;
extern crate handlebars;
extern crate image;
extern crate copy_dir;

#[path = "src/resources.rs"]
mod resources;

#[path = "src/tile_desc.rs"]
mod tile_desc;

#[path = "src/simple_file.rs"]
mod simple_file;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::collections::{HashMap, HashSet};

use handlebars::Handlebars;
use image::{FilterType, GenericImage, ColorType};
use resources::{RESOURCE_DIR, TILE_SHEET_IMAGE, TILE_SHEET_SPEC};
use tile_desc::TileDesc;

const COMPONENT_SPEC: &'static str = "components.toml";
const SPATIAL_HASH_SPEC: &'static str = "spatial_hash.toml";

const ENTITY_STORE_MACROS: &'static str = "entity_store_macros.rs";
const ENTITY_STORE_TEMPLATE: &'static str = "src/entity_store/template.rs.hbs";

const SPATIAL_HASH_MACROS: &'static str = "spatial_hash_macros.rs";
const SPATIAL_HASH_TEMPLATE: &'static str = "src/spatial_hash/template.rs.hbs";

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

fn read_entity_store_desc<P: AsRef<Path>>(path: P) -> EntityStoreDesc {
    simple_file::read_toml(path).expect("Failed to parse entity store desc")
}

fn read_spatial_hash_desc<P: AsRef<Path>>(path: P) -> SpatialHashDesc {
    simple_file::read_toml(path).expect("Failed to parse spatial hash desc")
}

fn make_handlebars() -> Handlebars {
    let mut handlebars = Handlebars::new();
    // prevent xml escaping
    handlebars.register_escape_fn(|input| input.to_string());
    handlebars
}

fn render_entity_system_template_internal<P: AsRef<Path>>(desc: EntityStoreDesc,
                                   template_path: P) -> String {

    let template = simple_file::read_string(template_path).expect("Failed to read template");

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

    let template = simple_file::read_string(template_path).expect("Failed to read template");

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

    let in_path = &resources::build_resource_path(COMPONENT_SPEC);
    let out_path = &resources::out_path(ENTITY_STORE_MACROS);

    let template_path = ENTITY_STORE_TEMPLATE;

    if source_changed_rel(in_path, out_path) || source_changed_rel(template_path, out_path) {
        let type_desc = read_entity_store_desc(in_path);
        let output = render_entity_system_template_internal(type_desc, template_path);
        simple_file::write_string(out_path, output).expect("Failed to write entity system code");
    }
}

fn render_spatial_hash_template() {
    let out_path = &PathBuf::from(&env::var("OUT_DIR").unwrap()).join(SPATIAL_HASH_MACROS);
    let in_path = &PathBuf::from(RESOURCE_DIR).join(SPATIAL_HASH_SPEC);
    let template_path = SPATIAL_HASH_TEMPLATE;
    let component_spec = &PathBuf::from(RESOURCE_DIR).join(COMPONENT_SPEC);

    if source_changed_rel(in_path, out_path) || source_changed_rel(template_path, out_path) {
        let desc = read_spatial_hash_desc(in_path);
        let type_desc = read_entity_store_desc(component_spec);
        let output = render_spatial_hash_template_internal(desc, type_desc, template_path);
        simple_file::write_string(out_path, output).expect("Failed to write spatial hash code");
    }
}

fn scale_tiles() {

    let in_path = &resources::build_resource_path(TILE_SHEET_IMAGE);
    let stage_path = &resources::stage_resource_path(TILE_SHEET_IMAGE);
    let run_path = &resources::resource_path(TILE_SHEET_IMAGE);

    let tiles: TileDesc = simple_file::read_toml(resources::build_resource_path(TILE_SHEET_SPEC))
        .expect("Failed to read tile spec");

    if source_changed_rel(in_path, stage_path) || source_changed_rel(in_path, run_path) {

        let original = image::open(in_path).expect(format!("Failed to open image: {:?}", in_path).as_ref());

        let (width, height) = original.dimensions();
        let scaled = original.resize(width * tiles.tile_scale,
                                           height * tiles.tile_scale,
                                           FilterType::Nearest).to_rgba();

        let (width, height) = scaled.dimensions();
        image::save_buffer(stage_path, &scaled, width, height, ColorType::RGBA(8))
            .expect(format!("Failed to save scaled image: {:?}", stage_path).as_ref());
    }
}

fn copy_tile_spec() {

    let in_path = &resources::build_resource_path(TILE_SHEET_SPEC);
    let out_path = &resources::stage_resource_path(TILE_SHEET_SPEC);

    fs::copy(in_path, out_path).expect("Failed to copy tile sheet spec");
}

fn make_resource_dir() {
    let resource_path = &resources::stage_resource_dir_path();
    if !resource_path.exists() {
        fs::create_dir(resource_path).expect("Failed to create resource dir");
    }
}

fn copy_resource_dir() {

    let resource_path = &resources::stage_resource_dir_path();

    let target = env::var("TARGET").unwrap();
    let host = env::var("HOST").unwrap();
    let profile = env::var("PROFILE").unwrap();

    let dests = if target == host {
        vec![
           Path::new("target").join(&profile),
           Path::new("target").join(&target).join(&profile),
        ]
    } else {
        vec![Path::new("target").join(&target).join(&profile)]
    };

    for dest in dests {
        if !dest.exists() {
            continue;
        }

        let dest_resource_path = dest.join(RESOURCE_DIR);

        if dest_resource_path.exists() {
            fs::remove_dir_all(&dest_resource_path).expect("Failed to remove old resources directory");
        }

        copy_dir::copy_dir(resource_path, dest_resource_path).expect("Failed to copy resource dir");
    }
}

fn main() {

    render_entity_system_template();
    render_spatial_hash_template();

    make_resource_dir();

    scale_tiles();
    copy_tile_spec();

    copy_resource_dir();
}
