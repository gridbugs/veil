// Generated file. Do not edit!

macro_rules! imports {
    () => {
        use cgmath::Vector2;
    }
}

macro_rules! position_type {
    () => {
        Vector2<i32>
    }
}

macro_rules! spatial_hash_cell_decl {
    ($SpatialHashCell:ident) => {
        pub struct $SpatialHashCell {
            pub solid_count: usize,
            pub opacity_total: f64,
            pub enemy_set: HashSet<EntityId>,
            pub entities: HashSet<EntityId>,
            pub last_updated: u64,

        }
    }
}

macro_rules! spatial_hash_cell_cons {
    ($SpatialHashCell:ident) => {
        $SpatialHashCell {
            solid_count: 0,
            opacity_total: 0.0,
            enemy_set: HashSet::new(),
            entities: HashSet::new(),
            last_updated: 0,
        }
    }
}
