extern crate gtk;
extern crate hex;
extern crate base64;
extern crate argon2;
extern crate crypto;
extern crate rand;

mod view;
mod domain;
mod entity;
mod util;
mod service;

use view::PassKeeperView;
use service::PassKeeperService;


fn main() {
  let view = PassKeeperView::new(PassKeeperService::new());
  view.show_gui();
}
