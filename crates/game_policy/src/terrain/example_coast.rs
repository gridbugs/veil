use rand::Rng;
use cgmath::Vector2;
use game_data::entity_store::EntityStoreChange;
use game_data::entity_id_allocator::EntityIdAllocator;
use game_data::content::prototypes;
use terrain::TerrainMetadata;
use geometry::perlin::{PerlinGrid, PerlinWrapType};

const WATER_ZOOM: usize = 10;
const WATER_ZOOM_F: f64 = WATER_ZOOM as f64;

pub fn generate<R: Rng>(width: usize, height: usize,
                        change: &mut EntityStoreChange,
                        allocator: &mut EntityIdAllocator,
                        rng: &mut R) -> TerrainMetadata {

    let water_perlin = PerlinGrid::new(width / WATER_ZOOM, height / WATER_ZOOM, PerlinWrapType::Regenerate, rng);

    let player_id = allocator.allocate();
    prototypes::player(change, player_id, Vector2::new(width as i32 / 2, height as i32 / 2));

    for y in 0..(height as i32) {
        for x in 0..(width as i32) {
            let water_perlin_coord = (x as f64 / WATER_ZOOM_F, y as f64 / WATER_ZOOM_F);

            if let Some(water_noise) = water_perlin.noise(water_perlin_coord.0, water_perlin_coord.1) {

                if water_noise >= -0.2 && water_noise <= 0.2 {
                    prototypes::water(change, allocator.allocate(), Vector2::new(x, y), rng);
                    continue;
                }
            }

            prototypes::stone_floor(change, allocator.allocate(), Vector2::new(x, y));

            if rng.next_f64() < 0.1 {
                prototypes::rain(change, allocator.allocate(), Vector2::new(x, y), rng);
            }
        }
    }

    TerrainMetadata {
        player_id: Some(player_id),
    }
}
