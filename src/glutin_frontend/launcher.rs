use glutin_frontend::frontend;
use game_policy::launch;

pub fn launch() {
    let (mut renderer, mut input) = frontend::create();
    launch::launch(&mut renderer, &mut input);
}
