macro_rules! migrate_data_copy {
    ($source:expr, $dest:expr, $dest_change:expr, $entity:expr, $component:ident) => {
        if let Some(x) = $source.$component.get(&$entity) {
            $dest_change.insertions.$component.insert($entity, x.clone());
        } else {
            $dest_change.removals.insert($entity, ComponentType::$component());
        }
    }
}

macro_rules! migrate_flag_copy {
    ($source:expr, $dest:expr, $dest_change:expr, $entity:expr, $component:ident) => {
        if $source.$component.contains(&$entity) {
            $dest_change.insertions.$component.insert($entity);
        } else {
            $dest_change.removals.insert($entity, ComponentType::$component());
        }
    }
}

macro_rules! migrate_data_move {
    ($source:expr, $dest:expr, $source_change:expr, $dest_change:expr, $entity:expr, $component:ident) => {
        if let Some(x) = $source.$component.get(&$entity) {
            $dest_change.insertions.$component.insert($entity, x.clone());
            $source_change.removals.insert($entity, ComponentType::$component());
        } else {
            $dest_change.removals.insert($entity, ComponentType::$component());
        }
    }
}

macro_rules! migrate_flag_move {
    ($source:expr, $dest:expr, $source_change:expr, $dest_change:expr, $entity:expr, $component:ident) => {
        if $source.$component.contains(&$entity) {
            $dest_change.insertions.$component.insert($entity);
            $source_change.removals.insert($entity, ComponentType::$component());
        } else {
            $dest_change.removals.insert($entity, ComponentType::$component());
        }
    }
}

macro_rules! migrate_data_swap {
    ($source:expr, $dest:expr, $source_change:expr, $dest_change:expr, $entity:expr, $component:ident) => {
        if let Some(x) = $source.$component.get(&$entity) {
            $dest_change.insertions.$component.insert($entity, x.clone());
        } else {
            $dest_change.removals.insert($entity, ComponentType::$component());
        }
        if let Some(x) = $dest.$component.get(&$entity) {
            $source_change.insertions.$component.insert($entity, x.clone());
        } else {
            $source_change.removals.insert($entity, ComponentType::$component());
        }
    }
}

macro_rules! migrate_flag_swap {
    ($source:expr, $dest:expr, $source_change:expr, $dest_change:expr, $entity:expr, $component:ident) => {
        if $source.$component.contains(&$entity) {
            $dest_change.insertions.$component.insert($entity);
        } else {
            $dest_change.removals.insert($entity, ComponentType::$component());
        }
        if $dest.$component.contains(&$entity) {
            $source_change.insertions.$component.insert($entity);
        } else {
            $source_change.removals.insert($entity, ComponentType::$component());
        }
    }
}
