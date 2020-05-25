use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

mod board;

use board::*;

mod tile;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}


const GRID_WIDTH: usize = 16;
const GRID_HEIGHT: usize = 16;
const PIXELS_PER_SQUARE_SIDE: f64 = 20.0;


#[wasm_bindgen(start)]
pub fn start() {
    log("running");
    log(&format!("yo: 5 % 2 = {} and -5 % 2 = {}", 5 % 2, -5 % 2));
    console_error_panic_hook::set_once();
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let mut board = Board::new(40, GRID_WIDTH, GRID_HEIGHT);
    render_grid(&context, &board);

    let mut game_running = false;
    let click_handler = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        event.prevent_default();
        let x = event.offset_x();
        let y = event.offset_y();
        log(&format!("x: {}, y: {}", x, y));
        log(&format!("{:?}", event));
        if !game_running {
            board.place_mines();
            game_running = true;
            log("mines placed");
        } else {
            // update game state
            log("game is running - updateBoardState()");
        }
    }) as Box<dyn FnMut(_)>);

    let context_menu_cb = Closure::wrap(
        Box::new(move |event: web_sys::MouseEvent| {
            event.prevent_default();
        }) as Box<dyn FnMut(_)>);

    canvas.add_event_listener_with_callback(
        "mousedown", click_handler.as_ref().unchecked_ref(),
    ).expect("Failed to add the mousedown event listener");
    click_handler.forget();

    canvas.add_event_listener_with_callback(
        "contextmenu", context_menu_cb.as_ref().unchecked_ref(),
    ).expect("Failed to add the contextmenu event listener.");
    context_menu_cb.forget();
}

fn render_grid(context: &web_sys::CanvasRenderingContext2d, board: &Board) {
    let num_cells = board.get_total_number_of_cells();
    for i in 0..num_cells {
        let x = i % GRID_WIDTH;
        let y = i / GRID_WIDTH;
        let mut fill_style: JsValue;
        if board.cells[i as usize].has_mine {
            fill_style = JsValue::from_str("rgba(255,0,255,1)");
        } else {
            fill_style = JsValue::from_str("rgba(125,50,255,1)");
        }
        context.set_fill_style(&fill_style);
        context.fill_rect(x as f64 * PIXELS_PER_SQUARE_SIDE,
                          y as f64 * PIXELS_PER_SQUARE_SIDE,
                          PIXELS_PER_SQUARE_SIDE,
                          PIXELS_PER_SQUARE_SIDE,
        );
        context.stroke_rect(x as f64 * PIXELS_PER_SQUARE_SIDE,
                            y as f64 * PIXELS_PER_SQUARE_SIDE,
                            PIXELS_PER_SQUARE_SIDE,
                            PIXELS_PER_SQUARE_SIDE,
        );
    }
}
