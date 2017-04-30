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

macro_rules! position {
    ($store:expr) => {
        $store.position
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

macro_rules! remove_implicit {
    ($self:expr, $entity_id:expr, $store:expr, $change:expr) => {
        if !$change.removals.contains($entity_id, ComponentType::Opacity) {
            if let Some(v) = $store.opacity.get(&$entity_id) {
                $self.opacity_total -= *v;
            }
        }
        if !$change.removals.contains($entity_id, ComponentType::Solid) {
            if $store.solid.contains(&$entity_id) {
                $self.solid_count -= 1;
            }
        }
        if !$change.removals.contains($entity_id, ComponentType::Enemy) {
            if $store.enemy.contains(&$entity_id) {
                $self.enemy_set.remove(&$entity_id);
            }
        }
    }
}

macro_rules! insert_implicit {
    ($self:expr, $entity_id:expr, $store:expr, $change:expr) => {
        if !$change.removals.contains($entity_id, ComponentType::Opacity) {
            if let Some(v) = $store.opacity.get(&$entity_id) {
                $self.opacity_total += *v;
            }
        }
        if !$change.removals.contains($entity_id, ComponentType::Solid) {
            if $store.solid.contains(&$entity_id) {
                $self.solid_count += 1;
            }
        }
        if !$change.removals.contains($entity_id, ComponentType::Enemy) {
            if $store.enemy.contains(&$entity_id) {
                $self.enemy_set.insert($entity_id);
            }
        }
    }
}

macro_rules! update_match_stmt {
    ($component_type:expr, $cell:expr, $entity_id:expr, $store:expr, $change:expr) => {
        match $component_type {
            ComponentType::Position => {
                $cell.remove_implicit($entity_id, $store, $change);
            }
            ComponentType::Opacity => {
                if let Some(v) = $store.opacity.get(&$entity_id) {
                    $cell.opacity_total -= *v;
                }
            }
            ComponentType::Solid => {
                if $store.solid.contains(&$entity_id) {
                    $cell.solid_count -= 1;
                }
            }
            ComponentType::Enemy => {
                if $store.enemy.contains(&$entity_id) {
                    $cell.enemy_set.remove(&$entity_id);
                }
            }
            _ => {
                // prevent the last_updated field from being changed
                continue;
            }
        }
    }
}

macro_rules! update_component_loops {
    ($self:expr, $store:expr, $change:expr, $time:expr) => {
        for (entity_id, new) in $change.insertions.opacity.iter() {
            if let Some(position) = post_change_get!($store, $change, *entity_id, position) {
                let old = if $change.removals.contains(*entity_id, ComponentType::Opacity) {
                    0.0
                } else {
                    $store.opacity.get(entity_id).map(Clone::clone).unwrap_or(0.0)
                };
                let increase = new - old;
                if let Some(mut cell) = $self.grid.get_mut(*position) {
                    cell.opacity_total += increase;
                    cell.last_updated = $time;
                }
            }
        }

        for entity_id in $change.insertions.solid.iter() {
            if let Some(position) = post_change_get!($store, $change, *entity_id, position) {
                if !$store.solid.contains(&entity_id) || $change.removals.contains(*entity_id, ComponentType::Solid) {
                    if let Some(mut cell) = $self.grid.get_mut(*position) {
                        cell.solid_count += 1;
                        cell.last_updated = $time;
                    }
                }
            }
        }

        for entity_id in $change.insertions.enemy.iter() {
            if let Some(position) = post_change_get!($store, $change, *entity_id, position) {
                if !$store.enemy.contains(&entity_id) || $change.removals.contains(*entity_id, ComponentType::Enemy) {
                    if let Some(mut cell) = $self.grid.get_mut(*position) {
                        cell.enemy_set.insert(*entity_id);
                        cell.last_updated = $time;
                    }
                }
            }
        }
    }
}
