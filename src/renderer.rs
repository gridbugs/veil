use knowledge::PlayerKnowledgeGrid;
use render_overlay::RenderOverlay;
use cgmath::Vector2;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GameRendererConfig {
    pub diminishing_lighting: bool,
}

impl Default for GameRendererConfig {
    fn default() -> Self {
        GameRendererConfig {
            diminishing_lighting: true,
        }
    }
}

pub trait GameRenderer {
    fn clear(&mut self);
    fn update_player_position(&mut self, player_coord: Vector2<i32>);
    fn update(&mut self, knowledge: &PlayerKnowledgeGrid, time: u64);
    fn draw(&mut self);
    fn draw_overlay(&mut self, overlay: RenderOverlay);
    fn publish(&mut self);
    fn set_config(&mut self, config: GameRendererConfig);
    fn config(&self) -> GameRendererConfig;
}
