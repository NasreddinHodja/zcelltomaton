use nannou::prelude::*;
use nannou::winit::event::MouseButton;

const WINDOW_HEIGHT: u32 = 800;
const WINDOW_WIDTH: u32 = 800;
const BOARD_ROWS: usize = 5;
const BOARD_COLS: usize = 5;

type State = usize;
struct Board {
    cells: Vec<Vec<State>>,
    rows: usize,
    cols: usize,
}

impl Board {
    fn new(rows: usize, cols: usize) -> Board {
        Board {
            cells: vec![vec![0; cols]; rows],
            rows,
            cols,
        }
    }

    fn position_to_rc(
        &self,
        x: f32,
        y: f32,
        cell_w: f32,
        cell_h: f32,
        window_w: f32,
        window_h: f32,
    ) -> Vec2 {
        // let r = (window_h / 2.0 - y) / self.rows as f32;
        // let c = (x + window_w / 2.0) / self.cols as f32;
        let r = ((-y + window_h / 2.0) / cell_h).floor();
        let c = ((x + window_w / 2.0) / cell_w).floor();

        Vec2::new(r, c)
    }

    fn set_cell(&mut self, r: usize, c: usize, state: State) {
        self.cells[r][c] = state
    }
}

const STATE_COLORS: [Srgb<u8>; 2] = [BLACK, PLUM];

struct Model {
    board: Board,
    window_id: WindowId,
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .size_pixels(WINDOW_WIDTH, WINDOW_HEIGHT)
        .view(view)
        .build()
        .unwrap();

    let board = Board::new(BOARD_ROWS, BOARD_COLS);

    Model { board, window_id }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn handle_mousepress(button: MouseButton, app: &App, model: &mut Model) {
    let win = app.window_rect();
    let cell_w = win.w() / BOARD_COLS as f32;
    let cell_h = win.h() / BOARD_ROWS as f32;
    let window_w = app.window_rect().w();
    let window_h = app.window_rect().h();
    let pos = app.mouse.position();
    let pos = model
        .board
        .position_to_rc(pos.x, pos.y, cell_h, cell_w, window_w, window_h);
    let mut state = 0;
    match button {
        MouseButton::Left => state = 1,
        MouseButton::Right => state = 0,
        _ => {}
    }
    model.board.set_cell(pos.x as usize, pos.y as usize, state);
}

fn event(app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent {
            id: _,
            simple: Some(event),
        } => match event {
            // TODO: fn handle_mousepress()
            MousePressed(MouseButton::Left) => {
                handle_mousepress(MouseButton::Left, &app, model);
            }
            MousePressed(MouseButton::Right) => {
                handle_mousepress(MouseButton::Right, &app, model);
            }
            _ => {}
        },
        // Event::DeviceEvent(_, _) => todo!(),
        // Event::Update(_) => todo!(),
        // Event::Suspended => todo!(),
        // Event::Resumed => todo!(),
        _ => {}
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(STATE_COLORS[0]);

    let win = app.window_rect();
    let cell_w = win.w() / BOARD_COLS as f32;
    let cell_h = win.h() / BOARD_ROWS as f32;

    for r in 0..BOARD_ROWS {
        for c in 0..BOARD_COLS {
            let x = -(win.w() / 2.0) + cell_w * c as f32 + cell_w / 2.0;
            let y = win.h() / 2.0 - cell_h * r as f32 - cell_h / 2.0;
            draw.rect()
                .x_y(x, y)
                .w_h(cell_w, cell_h)
                .color(STATE_COLORS[model.board.cells[r][c]]);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model).update(update).event(event).run();
}
