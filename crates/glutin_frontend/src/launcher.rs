use game_policy::launch;

use frontend;

pub fn launch() {
    let (mut renderer, mut input) = frontend::create();
    launch::launch(&mut renderer, &mut input);
}
