use glutin_frontend::frontend;
use launch;

pub fn launch() {
    let (mut renderer, mut input) = frontend::create();
    launch::launch(&mut renderer, &mut input);
}
