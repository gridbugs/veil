use sdl2::rect::Rect;
use cgmath::Vector2;
use grid::{StaticGrid, StaticGridIdx, static_grid};
use content::{ComplexTile, OverlayType};
use sdl2_frontend::tile;
use knowledge::{PlayerKnowledgeGrid, PlayerKnowledgeTile};

#[derive(Debug)]
pub struct TileBufferCell {
    pub channels: [Option<Rect>; tile::NUM_TILE_CHANNELS],
    pub visible: bool,
    priorities: [u8; tile::NUM_TILE_CHANNELS],
}

#[derive(Debug)]
pub struct TileBuffer {
    grid: StaticGrid<TileBufferCell>,
}

impl Default for TileBufferCell {
    fn default() -> Self {
        TileBufferCell {
            channels: [None; tile::NUM_TILE_CHANNELS],
            visible: true,
            priorities: [0; tile::NUM_TILE_CHANNELS],
        }
    }
}

impl TileBufferCell {
    fn clear(&mut self) {
        self.channels = [None; tile::NUM_TILE_CHANNELS];
        self.visible = false;
        self.priorities = [0; tile::NUM_TILE_CHANNELS];
    }

    fn update(&mut self, tile: &tile::Tile, priority: u8) {
        for &tile::Channel { id, sprite } in tile.channels.iter() {
            if priority >= self.priorities[id] {
                self.priorities[id] = priority;
                self.channels[id] = Some(sprite);
            }
        }
    }
}

pub type Iter<'a> = static_grid::Iter<'a, TileBufferCell>;
pub type CoordIter = static_grid::CoordIter;

impl TileBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        TileBuffer {
            grid: StaticGrid::new_default(width, height),
        }
    }

    pub fn get<I: StaticGridIdx>(&self, index: I) -> Option<&TileBufferCell> {
        self.grid.get(index)
    }

    pub fn iter(&self) -> Iter {
        self.grid.iter()
    }

    pub fn coord_iter(&self) -> CoordIter {
        self.grid.coord_iter()
    }

    pub fn update(&mut self, knowledge: &PlayerKnowledgeGrid, resolver: &tile::TileResolver, time: u64) {
        for (coord, mut cell) in izip!(self.grid.coord_iter(), self.grid.iter_mut()) {
            cell.clear();
            if let Some(knowledge_cell) = knowledge.get(coord) {
                cell.visible = knowledge_cell.last_updated == time;
                if cell.visible {
                    if knowledge_cell.veil_cell.current && knowledge_cell.veil_cell.next {
                        cell.channels[tile::OVERLAY_CHANNEL] = Some(*resolver.resolve_overlay(OverlayType::Veil));
                    } else if knowledge_cell.veil_cell.current {
                        cell.channels[tile::OVERLAY_CHANNEL] = Some(*resolver.resolve_overlay(OverlayType::VeilCurrent));
                    } else if knowledge_cell.veil_cell.next {
                        cell.channels[tile::OVERLAY_CHANNEL] = Some(*resolver.resolve_overlay(OverlayType::VeilNext));
                    } else {
                        cell.channels[tile::OVERLAY_CHANNEL] = None;
                    }
                }
                for &PlayerKnowledgeTile { priority, tile, forgetable } in knowledge_cell.tiles.iter() {
                    if !cell.visible && forgetable {
                        continue;
                    }
                    let simple_tile = match tile {
                        ComplexTile::Wall { front, top } => {
                            let south_coord = coord + Vector2::new(0, 1);
                            if let Some(south_cell) = knowledge.get(south_coord) {
                                if south_cell.wall {
                                    top
                                } else {
                                    front
                                }
                            } else {
                                // bottom of map
                                front
                            }
                        }
                        ComplexTile::Simple(tile) => {
                            tile
                        }
                    };

                    cell.update(resolver.resolve_tile(simple_tile), priority);
                }
            }
        }
    }
}
