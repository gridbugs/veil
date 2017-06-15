use entity_store::{EntityId, EntityStore};
use spatial_hash::SpatialHashCell;
use grid::StaticGrid;
use content::{ComplexTile, OverlayType, TileType};
use knowledge::KnowledgeGrid;
use observation::ObservationMetadata;
use coord::LookupCoord;
use veil_state::VeilCell;
use cgmath::Vector2;

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
    pub low_tile: bool,
    pub tile_front: Option<TileType>,
    pub solid: bool,
    pub door: Option<EntityId>,
    pub enemy: Option<EntityId>,
    pub player: bool,
    pub veil_cell: VeilCell,
}

#[derive(Debug)]
pub struct PlayerKnowledgeGrid {
    last_updated: u64,
    current_time: u64,
    player_coord: Option<Vector2<i32>>,
    last_player_coord: Option<Vector2<i32>>,
    grid: StaticGrid<PlayerKnowledgeCell>,
}

impl Default for PlayerKnowledgeCell {
    fn default() -> Self {
        PlayerKnowledgeCell {
            last_updated: 0,
            tiles: Vec::new(),
            overlay: None,
            wall: false,
            low_tile: false,
            tile_front: None,
            solid: false,
            door: None,
            enemy: None,
            player: false,
            veil_cell: Default::default(),
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
            self.enemy = spatial_hash_cell.enemy_set.iter().next().cloned();
            self.player = spatial_hash_cell.player_count > 0;
            self.veil_cell.current = spatial_hash_cell.veil_current_count > 0;
            self.veil_cell.next = spatial_hash_cell.veil_next_count > 0;
            self.low_tile = spatial_hash_cell.low_tile_count > 0;
            self.tile_front = spatial_hash_cell.tile_front_set.iter().next()
                .and_then(|id| entity_store.tile_front.get(id)).cloned();

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
            current_time: 0,
            player_coord: None,
            last_player_coord: None,
            grid: StaticGrid::new_default(width, height),
        }
    }

    pub fn get(&self, coord: Vector2<i32>) -> Option<&PlayerKnowledgeCell> {
        self.grid.get(coord)
    }

    pub fn is_visible(&self, coord: Vector2<i32>, time: u64) -> bool {
        self.get(coord).map(|c| c.is_visible(time)).unwrap_or(false)
    }

    pub fn player_coord(&self) -> Option<Vector2<i32>> {
        self.player_coord
    }

    pub fn last_player_coord(&self) -> Option<Vector2<i32>> {
        self.last_player_coord
    }

    pub fn clear_last_player_coord(&mut self) {
        self.last_player_coord = None;
    }
}

impl KnowledgeGrid for PlayerKnowledgeGrid {
    fn update_cell(&mut self, coord: Vector2<i32>, spatial_hash_cell: &SpatialHashCell,
                   entity_store: &EntityStore) -> ObservationMetadata {

        if let Some(knowledge_cell) = self.grid.get_mut(coord) {
            if knowledge_cell.last_updated == self.current_time {
                return Default::default();
            }

            if self.last_updated != self.current_time {
                self.last_updated = self.current_time;
            }

            if spatial_hash_cell.player_count > 0 {
                self.player_coord = Some(coord);
                self.last_player_coord = Some(coord);
            } else if let Some(player_coord) = self.player_coord {
                if player_coord == coord {
                    // there's no player here, but we previously saw the player here
                    self.player_coord = None;
                }
            }

            knowledge_cell.update(spatial_hash_cell, entity_store, self.current_time)
        } else {
            Default::default()
        }
    }

    fn set_time(&mut self, time: u64) {
        self.current_time = time;
    }
}

impl LookupCoord for PlayerKnowledgeGrid {
    type Item = PlayerKnowledgeCell;
    fn lookup_coord(&self, coord: Vector2<i32>) -> Option<&Self::Item> {
        self.get(coord)
    }
}
