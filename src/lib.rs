extern crate rand;
extern crate stdweb;
#[macro_use]
extern crate log;
#[macro_use]
extern crate yew;

use std::option::Option;
use rand::prelude::*;
use yew::prelude::*;

pub type Context = ();

type Board = Vec<Vec<String>>;
type Source = Vec<String>;
type Matched = Vec<String>;

#[derive(Debug)]
pub struct Position {
    x: i8,
    y: i8,
}

impl Position {
    fn new(x: i8, y: i8) -> Self {
        Position { x, y }
    }
}

pub struct Model {
    width:  i8,
    height: i8,
    board: Board,
    source: Source,
    position: Option<Position>,
}

impl Model {
    fn empty(width: i8, height: i8) -> Self {
        Self {
            width,
            height,
            board: empty_board(width, height),
            source: vec![
                "a".to_owned(),
                "b".to_owned(),
                "c".to_owned(),
                "d".to_owned(),
                "e".to_owned(),
            ],
            position: None,
        }
    }
}

pub enum Msg {
    Roll,
    MouseDown,
    MouseUp,
    MouseMove,
}

fn empty_board(width: i8, height: i8) -> Board {
    let mut board = Vec::new();
    for _i in 0..width {
        let mut line = Vec::new();
        for _j in 0..height {
            line.push("".to_owned());
        }
        board.push(line);
    }
    board
}

fn generate_cell(full_source: &Source, matched: Matched) -> String {
    let mut rng = thread_rng();
    let source = get_source(full_source, matched);
    let index = rng.gen_range(0, source.len());
    source[index].to_owned()
}

fn get_source(full_source: &Source, matched: Matched) -> Vec<String> {
    full_source.iter().filter(|i| {
        for m in matched.iter() {
            if &m == i {
                return false;
            }
        }
        true
    }).map(|i| i.to_string()).collect()
}

fn generate_board(board: &mut Board, full_source: &Source) {
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            let mut matched = Vec::new();
            if i > 1 && board[i - 1][j] == board[i - 2][j] {
                matched.push(board[i - 1][j].clone());
            }
            if j > 1 && board[i][j - 1] == board[i][j - 2] {
                matched.push(board[i][j - 1].clone())
            }
            board[i][j] = generate_cell(full_source, matched);
        }
    }
}

impl Component<Context> for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<Context, Self>) -> Self {
        let mut model = Model::empty(10, 10);
        generate_board(&mut model.board, &model.source);
        model
    }

    fn update(&mut self, msg: Self::Message, _env: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::Roll => {
                info!("roll desu");
                generate_board(&mut self.board, &self.source);
                debug!("board is {:?}", self.board);
            },
            Msg::MouseDown => {
                info!("mouse down");
            },
            _ => {
                info!("something else");
            }
        }
        true
    }
}

impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {
        html! {
            <div
                style=format!("\
                    width: {}px;\
                    margin: 0 auto;\
                ", self.board.len() * 34),>
                { for self.board.iter().map(line) }
            </div>
        }
    }
}

fn line(list: &Vec<String>) -> Html<Context, Model> {
    html! {
        <div>{ for list.iter().map(cell) }</div>
    }
}

fn cell(value: &String) -> Html<Context, Model> {
    html! {
        <div
            style="\
                display: inline-block;\
                width: 30px;\
                height: 30px;\
                line-height: 30px;\
                text-align: center;\
                border: 1px solid black;\
                background: yellow;\
                color: green;\
                margin: 1px;\
                cursor: pointer;\
                user-select: none;\
                -moz-user-select: none;\
                ",
                onmousedown=|_| Msg::Roll,
        >
            { value }
        </div>
    }
}
