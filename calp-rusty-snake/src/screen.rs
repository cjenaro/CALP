use model::World;

pub trait Screen {
    fn refresh_screen(&self, world: &World);
}
