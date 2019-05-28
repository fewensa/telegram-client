use rtdlib::types::Update;

pub struct Listener {
  lin_update: Option<Box<Fn(Box<Update>) + 'static>>
}

impl Listener {
  pub fn new() -> Self {
    Self {
      lin_update: None
    }
  }

//  pub fn lout(self) -> Lout {
//    Lout::new(self)
//  }

  pub fn on_update<F>(&mut self, fnc: F) -> &mut Self where F: Fn(Box<Update>) + 'static {
    self.lin_update = Some(Box::new(fnc));
    self
  }
}

pub struct Lout {
  listener: Listener
}

impl Lout {
  fn new(listener: Listener) -> Self {
    Lout { listener }
  }
}
