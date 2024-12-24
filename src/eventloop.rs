use crate::frame::Frame;

trait LoopState {
    fn update(self) -> Self;
    fn draw<const W:  usize, const H: usize>(&self) -> Frame<W, H>;
}