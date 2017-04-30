macro_rules! post_change_get {
    ($store:expr, $change:expr, $entity_id:expr, $field:ident) => {
        $change.insertions.$field.get(&$entity_id).or_else(|| {
            if $change.removals.contains($entity_id, ComponentType::$field()) {
                None
            } else {
                $store.$field.get(&$entity_id)
            }
        })
    }
}

macro_rules! post_change_contains_key {
    ($store:expr, $change:expr, $entity_id:expr, $field:ident) => {
        $change.insertions.$field.contains_key(&$entity_id) ||
            (!$change.removals.contains($entity_id, ComponentType::$field()) &&
             $store.$field.contains_key(&$entity_id))
    }
}

macro_rules! post_change_contains {
    ($store:expr, $change:expr, $entity_id:expr, $field:ident) => {
        $change.insertions.$field.contains(&$entity_id) ||
            (!$change.removals.contains($entity_id, ComponentType::$field()) &&
             $store.$field.contains(&$entity_id))
    }
}
