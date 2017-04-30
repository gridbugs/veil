use entity_store::*;

#[test]
fn commit_into() {

    let mut es0 = EntityStore::new();
    let mut es1 = EntityStore::new();

    let mut change = EntityStoreChange::new();

    let e0 = EntityId::new(0);

    change.insertions.solid.insert(e0);
    es0.commit(&mut change);

    assert!(es0.solid.contains(&e0));
    assert!(!es1.solid.contains(&e0));

    change.removals.insert(e0, ComponentType::Solid);
    es0.commit_into(&mut change, &mut es1);
    assert!(!es0.solid.contains(&e0));
    assert!(es1.solid.contains(&e0));
}
