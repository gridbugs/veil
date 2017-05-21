use entity_store::{EntityId, EntityStore};
use spatial_hash::SpatialHashCell;
use grid::{StaticGrid, StaticGridIdx};
use content::{ComplexTile, OverlayType};
use knowledge::KnowledgeGrid;
use observation::ObservationMetadata;

#[derive(Debug)]
pub struct PlayerKnowledgeTile {
    pub priority: u8,
    pub tile: ComplexTile,
    pub forgetable: bool,
}

#[derive(Debug)]
pub struct PlayerKnowledgeCell {
    pub last_updated: u64,
    pub tiles: Vec<PlayerKnowledgeTile>,
    pub overlay: Option<OverlayType>,
    pub wall: bool,
    pub solid: bool,
    pub door: Option<EntityId>,
}

#[derive(Debug)]
pub struct PlayerKnowledgeGrid {
    last_updated: u64,
    grid: StaticGrid<PlayerKnowledgeCell>,
}

impl Default for PlayerKnowledgeCell {
    fn default() -> Self {
        PlayerKnowledgeCell {
            last_updated: 0,
            tiles: Vec::new(),
            overlay: None,
            wall: false,
            solid: false,
            door: None,
        }
    }
}

impl PlayerKnowledgeCell {
    fn update(&mut self, spatial_hash_cell: &SpatialHashCell, entity_store: &EntityStore, time: u64) -> ObservationMetadata {

        let mut changed = false;

        if self.last_updated < spatial_hash_cell.last_updated {
            self.tiles.clear();
            self.wall = false;
            for entity_id in spatial_hash_cell.tile_set.iter() {
                if entity_store.invisible.contains(entity_id) {
                    continue;
                }
                if let Some(tile) = entity_store.tile.get(entity_id) {
                    if let Some(priority) = entity_store.tile_priority.get(entity_id) {
                        self.tiles.push(PlayerKnowledgeTile {
                            tile: *tile,
                            priority: *priority,
                            forgetable: entity_store.forgetable.contains(entity_id),
                        });
                        if tile.is_wall() {
                            self.wall = true;
                        }
                    }
                }
            }
            self.solid = spatial_hash_cell.solid_count > 0;
            self.door = spatial_hash_cell.door_set.iter().next().cloned();

            changed = true;
        }

        let md = ObservationMetadata {
            changed: changed,
            new: self.last_updated == 0,
        };

        self.last_updated = time;

        md
    }

    pub fn is_visible(&self, time: u64) -> bool {
        self.last_updated == time
    }
}

impl PlayerKnowledgeGrid {
    pub fn new(width: usize, height: usize) -> Self {
        PlayerKnowledgeGrid {
            last_updated: 0,
            grid: StaticGrid::new_default(width, height),
        }
    }

    pub fn get<I: StaticGridIdx>(&self, coord: I) -> Option<&PlayerKnowledgeCell> {
        self.grid.get(coord)
    }

    pub fn is_visible<I: StaticGridIdx>(&self, coord: I, time: u64) -> bool {
        self.get(coord).map(|c| c.is_visible(time)).unwrap_or(false)
    }
}

impl KnowledgeGrid for PlayerKnowledgeGrid {
    fn update_cell<I: StaticGridIdx>(&mut self, coord: I, spatial_hash_cell: &SpatialHashCell,
                                         entity_store: &EntityStore, time: u64) -> ObservationMetadata {

        if let Some(knowledge_cell) = self.grid.get_mut(coord) {
            if knowledge_cell.last_updated == time {
                return Default::default();
            }

            if self.last_updated != time {
                self.last_updated = time;
            }

            knowledge_cell.update(spatial_hash_cell, entity_store, time)
        } else {
            Default::default()
        }
    }
}
