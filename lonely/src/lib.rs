use engine;
use wasm_bindgen::JsCast;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

type Result<T> = std::result::Result<T, JsValue>;

fn window() -> Result<web_sys::Window> {
    match web_sys::window() {
        Some(window) => Ok(window),
        None => Err("Window not found.".into()),
    }
}

fn document() -> Result<web_sys::Document> {
    let window = window()?;
    match window.document() {
        Some(document) => Ok(document),
        None => Err("Document not found.".into()),
    }
}

fn body() -> Result<web_sys::HtmlElement> {
    let document = document()?;
    match document.body() {
        Some(body) => Ok(body),
        None => Err("Body not found.".into()),
    }
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) -> Result<i32> {
    let window = window()?;
    window.request_animation_frame(f.as_ref().unchecked_ref())
}

#[wasm_bindgen]
pub fn run() -> Result<()> {
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    let mut i = 0;
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        match body() {
            Ok(body) => {
                if i > 300 {
                    body.set_text_content(Some("All done!"));

                    // Drop our handle to this closure so that it will get cleaned
                    // up once we return.
                    let _ = f.borrow_mut().take();
                    return;
                }

                // Set the body's text content to how many times this
                // requestAnimationFrame callback has fired.
                i += 1;
                let text = format!("requestAnimationFrame has been called {} times.", i);
                body.set_text_content(Some(&text));
                // Schedule ourself for another requestAnimationFrame callback.
                match request_animation_frame(f.borrow().as_ref().unwrap()) {
                    Ok(time) => web_sys::console::log_1(&format!("time: {}", time).into()),
                    Err(err) => web_sys::console::log_1(&err),
                }
            }
            Err(err) => {
                web_sys::console::log_1(&err);
                let _ = f.borrow_mut().take();
                return;
            }
        }
    }) as Box<dyn FnMut()>));
    request_animation_frame(g.borrow().as_ref().unwrap())?;
    Ok(())
}

pub fn engine_go() {
    match engine::run() {
        Ok(sound) => println!("{}", sound),
        Err(_err) => println!("Engine won't run ..."),
    }
}
