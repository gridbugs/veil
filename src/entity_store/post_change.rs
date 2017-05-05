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

macro_rules! post_change_get_ {
    ($store:expr, $change:expr, $entity_id:expr, $field:ident) => {
        $change.$field.get(&$entity_id).map(|change| {
            match change {
                &DataChangeType::Insert(ref x) => Some(x),
                &DataChangeType::Remove => None,
            }
        }).unwrap_or_else(|| {
            $store.$field.get(&$entity_id)
        })
    }
}
