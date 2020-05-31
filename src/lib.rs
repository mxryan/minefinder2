use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

mod board;

use board::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

const GRID_WIDTH: usize = 16;
const GRID_HEIGHT: usize = 16;
const CELL_LEN: f64 = 20.0;
const RGBA_MAGENTA: &str = "rgba(255,0,255,1)";
const RGBA_CYAN: &str = "rgba(0,255,255,1)";
const RGBA_WHITE: &str = "rgba(255,255,255,1)";


fn get_canvas() -> web_sys::HtmlCanvasElement {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    canvas
}

fn get_2d_context(canvas: &web_sys::HtmlCanvasElement)
                  -> web_sys::CanvasRenderingContext2d {
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context
}


#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();

    let canvas = get_canvas();
    let context = get_2d_context(&canvas);
    let mut game_running = false;
    let mut board = Board::new(40, GRID_WIDTH, GRID_HEIGHT);

    render_grid(&context, &board);

    let cloz = move |event: web_sys::MouseEvent| {
        let (x, y) = convert_coords(event.offset_x(), event.offset_y());
        let click = match event.button() {
            0 => Click::Left,
            2 => Click::Right,
            _ => return
        };

        log(&format!("x: {}, y: {}", x, y));

        if !game_running {
            board.place_mines(x as usize, y as usize);
            board.set_cells_num_bomb_neighbors();
            board.update_state(x as usize, y as usize, click);
            game_running = true;
        } else if game_running {
            board.update_state(x as usize, y as usize, click);
        }

        render_grid(&context, &board);
    };

    let click_handler = Closure::wrap(Box::new(cloz) as Box<dyn FnMut(_)>);

    let context_menu_cb = Closure::wrap(
        Box::new(|event: web_sys::MouseEvent| {
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
        let x = (i % GRID_WIDTH) as f64;
        let y = (i / GRID_WIDTH) as f64;

        // todo: this only needs to be called first time the board is drawn
        //  and instead of drawing each individual rect i could just draw a
        //  bunch of straight lines
        context.stroke_rect(x * CELL_LEN, y * CELL_LEN, CELL_LEN, CELL_LEN);


        let font = "14px serif";
        context.set_font(font);
        match board.cells[i].state {
            CellState::Hidden => {
                context.set_fill_style(&JsValue::from_str(RGBA_MAGENTA));
                context.fill_rect(x * CELL_LEN, y * CELL_LEN, CELL_LEN, CELL_LEN);
            }

            CellState::Flagged => {
                context.set_fill_style(&JsValue::from_str(RGBA_MAGENTA));
                context.fill_rect(x * CELL_LEN, y * CELL_LEN, CELL_LEN, CELL_LEN);
                context.stroke_text("F", x * CELL_LEN + CELL_LEN / 2.5,
                                    y * CELL_LEN + CELL_LEN / 1.5, );
            }

            CellState::Revealed => {
                context.set_fill_style(&JsValue::from_str(RGBA_CYAN));
                context.fill_rect(x * CELL_LEN, y * CELL_LEN, CELL_LEN, CELL_LEN);
                if board.cells[i].has_mine {
                    context.stroke_text("X", x * CELL_LEN + CELL_LEN / 2.5,
                                        y * CELL_LEN + CELL_LEN / 1.5, );
                } else {
                    context.stroke_text(
                        &format!("{}", board.cells[i].neighboring_mines),
                        x * CELL_LEN + CELL_LEN / 2.5,
                        y * CELL_LEN + CELL_LEN / 1.5,
                    );
                }
            }
        }
    }
}

/// Translates the click event coordinates to the relative coordinates used by
/// the game board.
fn convert_coords(x: i32, y: i32) -> (i32, i32) {
    (x / CELL_LEN as i32, y / CELL_LEN as i32)
}



