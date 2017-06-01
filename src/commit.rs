use knowledge::PlayerKnowledgeGrid;
use reaction::Reaction;
use entity_store::*;
use spatial_hash::*;
use entity_id_allocator::*;
use content::ActionType;
use observation::shadowcast::ShadowcastEnv;
use policy::*;
use renderer::GameRenderer;
use schedule::Schedule;

pub struct CommitEnv<'a, Ren: 'a + GameRenderer> {
    pub renderer: &'a mut Ren,
    pub change: &'a mut EntityStoreChange,
    pub entity_store: &'a mut EntityStore,
    pub spatial_hash: &'a mut SpatialHashTable,
    pub player_knowledge: &'a mut PlayerKnowledgeGrid,
    pub shadowcast: &'a mut ShadowcastEnv,
    pub time: &'a mut u64,
    pub reactions: &'a mut Vec<Reaction>,
    pub id_allocator: &'a mut EntityIdAllocator,
    pub policy: &'a GamePolicy,
    pub schedule: &'a mut Schedule<ActionType>,
}

impl<'a, Ren: GameRenderer> CommitEnv<'a, Ren> {
    pub fn commit(self, initial_action: ActionType) {
        self.reactions.clear();
        self.reactions.push(Reaction::immediate(initial_action));

        while let Some(reaction) = self.reactions.pop() {
            reaction.action.populate(self.change, self.entity_store);

            if self.policy.on_change(self.change, self.entity_store, self.spatial_hash, self.reactions) {
                *self.time += 1;
                self.spatial_hash.update(self.entity_store, self.change, *self.time);
                self.entity_store.commit_change(self.change);
            } else {
                self.change.clear();
            }
        }
    }
}
