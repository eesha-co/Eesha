use eesha::eesha_test;
use winit::event_loop::EventLoopWindowTarget;

fn smoke(_elwt: &EventLoopWindowTarget<()>) {}
fn other_smoke(_elwt: &EventLoopWindowTarget<()>) {}

eesha_test!(smoke, other_smoke);
