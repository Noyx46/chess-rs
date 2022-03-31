use gloo_console::log;
use gloo_utils;
use web_sys::HtmlElement;
use yew::prelude::*;

const BOARD_SIZE: usize = 8;

enum Msg {
    /// A right click or contextmenu event. Used for
    /// highlighting the board. Fields are for client
    /// x and y mouse positions.
    RightClick(u32, u32),
}

struct App {
    /// A chess board has dimensions of 8 by 8.
    /// The chess board will be stored in an array, with the
    /// first 8 elements composing the first row, the second
    /// 8 elements composing the second row, etc.
    _board: [u8; BOARD_SIZE * BOARD_SIZE],

    /// A NodeRef to the board in the HTML DOM so the board
    /// can pass back mouse coordinates
    board_html: NodeRef,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App {
            _board: [0; 64],
            board_html: NodeRef::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::RightClick(x, y) => {
                let style = gloo_utils::document()
                    .default_view()
                    .unwrap()
                    .get_computed_style(&gloo_utils::body())
                    .unwrap()
                    .unwrap();
                let tile_size = style.get_property_value("--c-tile-size").unwrap();
                log!("Right click", x, y, "; tile size: ", tile_size);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let board_ref = self.board_html.clone();
        let board_onclick = ctx.link().callback(move |e: MouseEvent| {
            e.prevent_default();
            let board = board_ref.cast::<HtmlElement>().unwrap();

            // Get bounding client rect
            let rect = board.get_bounding_client_rect();
            let mouse_x = ((e.client_x() as f64) - rect.left()) as u32;
            let mouse_y = ((e.client_y() as f64) - rect.top()) as u32;

            Msg::RightClick(mouse_x, mouse_y)
        });
        let board_html = Self::make_board_html();
        html! {
            <div
                ref={ self.board_html.clone() }
                oncontextmenu={ board_onclick }
                class="c-container"
            >
                { board_html }
            </div>
        }
    }
}

impl App {
    fn make_board_html() -> Html {
        let mut board = Vec::with_capacity(BOARD_SIZE);
        for row in 0..BOARD_SIZE {
            let mut board_row = Vec::with_capacity(BOARD_SIZE);
            for col in 0..BOARD_SIZE {
                let tile_color = if (row + col) % 2 == 0 {
                    "c-tile-white"
                } else {
                    "c-tile-black"
                };
                let board_tile = html! {
                    <div class={ classes!("c-tile", tile_color) }></div>
                };
                board_row.push(board_tile);
            }
            let board_row = html! {
                <div class="c-row">{ for board_row }</div>
            };
            board.push(board_row);
        }
        html! {
            <div
                class="c-board"
            >
                { for board }
            </div>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
