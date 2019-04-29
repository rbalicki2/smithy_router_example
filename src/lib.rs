#![feature(
  proc_macro_hygiene,
  slice_patterns,
  custom_attribute,
  extern_crate_item_prelude,
  bind_by_move_pattern_guards
)]

use wasm_bindgen::prelude::*;

mod util;

use self::util::{
  get_current_hash,
  get_location,
};
use smithy::{
  smd,
  smd_no_move,
  types::{
    Component,
    SmithyComponent,
  },
};
use std::{
  cell::RefCell,
  collections::HashMap,
  rc::Rc,
};
use web_sys::MouseEvent;

type ItemId = usize;

#[derive(Debug)]
enum Page {
  Home,
  ItemView(ItemId),
}

impl Page {
  pub fn go_to_detail_view(&mut self, id: ItemId) {
    *self = Page::ItemView(id);
    let _ = get_location().set_hash(&id.to_string());
  }

  pub fn go_home(&mut self) {
    *self = Page::Home;
    let _ = get_location().set_hash("");
  }

  pub fn handle_hash_change(&mut self) {
    let id_opt = get_current_hash().and_then(|hash| hash.parse::<ItemId>().ok());
    match id_opt {
      Some(id) => {
        self.go_to_detail_view(id);
      },
      None => {
        self.go_home();
      },
    };
  }
}

impl Default for Page {
  fn default() -> Page {
    let mut page = Page::Home;
    page.handle_hash_change();
    page
  }
}

#[derive(Debug)]
struct Item {
  pub text: String,
  pub detail_text: String,
}

fn get_items() -> HashMap<ItemId, Item> {
  let mut items = HashMap::new();
  items.insert(
    0,
    Item {
      text: "Earth".to_string(),
      detail_text: "Earth is the third planet from the Sun.".to_string(),
    },
  );
  items.insert(
    1,
    Item {
      text: "Mars".to_string(),
      detail_text: "Mars is the fourth planet from the Sun.".to_string(),
    },
  );
  items
}

pub fn render() -> impl Component {
  let mut page = Page::default();
  let items = get_items();

  smd!(
    on_hash_change={|_| page.handle_hash_change()};
    post_render={|| {
      web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!("page {:?}", page)));
    }};
    {
      match page {
        Page::Home => smd_no_move!(
          <h1>Planets</h1>
          <ul>
            {
              let ref mut page = page;
              let page = Rc::new(RefCell::new(page));
              let items = items.iter().map(|(id, item)| (id, item, page.clone())).collect::<Vec<_>>();
              &mut items.into_iter().map(|(id, item, page)| {
                smd!(<li>
                  <a
                    on_click={|e: &MouseEvent| {
                      page.borrow_mut().go_to_detail_view(*id);
                      e.prevent_default();
                    }}
                    href
                  >
                    { &item.text }
                  </a>
                </li>)
              }).collect::<Vec<SmithyComponent>>()
            }
          </ul>
        ),
        Page::ItemView(id) => {
          let ref mut page = page;
          let item_opt = items.get(&id);
          smd!(
            {
              match item_opt {
                Some(item) => smd!(
                  <h1>{ &item.text }</h1>
                  <p>{ &item.detail_text }</p>
                ),
                None => smd!(No item found),
              }
            }
            <a
              on_click={|e: &MouseEvent| {
                page.go_home();
                e.prevent_default();
              }}
              href
            >
              Back to list
            </a>
          )
        },
      }
    }
  )
}

#[wasm_bindgen]
pub fn start(root_element: web_sys::Element) {
  let app = render();

  smithy::mount(Box::new(app), root_element);
}
