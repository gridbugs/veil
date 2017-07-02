use glutin_frontend::frontend;

pub fn launch() {
    let (mut renderer, mut input) = frontend::create();
    frontend::example(&mut renderer, &mut input);
}
