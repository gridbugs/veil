use knowledge::PlayerKnowledgeGrid;
use render_overlay::RenderOverlay;

pub trait GameRendererGen {
    fn clear(&mut self);
    fn update(&mut self, knowledge: &PlayerKnowledgeGrid, time: u64);
    fn draw(&mut self);
    fn draw_overlay(&mut self, overlay: RenderOverlay);
    fn publish(&mut self);
}
