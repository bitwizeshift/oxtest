pub trait Fixture: Default {
  fn set_up(&mut self) {}

  fn tear_down(&mut self) {}
}
