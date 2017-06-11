use knowledge::PlayerKnowledgeGrid;
use render_overlay::RenderOverlay;
use cgmath::Vector2;

pub trait GameRenderer {
    fn clear(&mut self);
    fn update_player_position(&mut self, player_coord: Vector2<i32>);
    fn update(&mut self, knowledge: &PlayerKnowledgeGrid, time: u64);
    fn draw(&mut self);
    fn draw_overlay(&mut self, overlay: RenderOverlay);
    fn publish(&mut self);
}
