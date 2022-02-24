use cell::Cellule;
use gloo_timers::callback::Interval;
use rand::Rng;
use yew::html::Scope;
use yew::{classes, html, Component, Context, Html};

mod cell;

pub enum Msg {
    Random,
    Start,
    Step,
    Reset,
    Stop,
    IncreaseT,
    DecreaseT,
    ToggleCellule(usize),
    Tick,
}

pub struct App {
    active: bool,
    cellules: Vec<Cellule>,
    cellules_width: usize,
    cellules_height: usize,
    temp: f32,
    _interval: Interval,
}

impl App {
    pub fn random_mutate(&mut self) {
        for cellule in self.cellules.iter_mut() {
            if rand::thread_rng().gen() {
                cellule.set_alive();
            } else {
                cellule.set_dead();
            }
        }
    }

    fn reset(&mut self) {
        for cellule in self.cellules.iter_mut() {
            cellule.set_dead();
        }
    }

    fn step_ising(&mut self) {
        // pick a random cell and flip it with an ising probability
        let mut rng = rand::thread_rng();
        let row = rng.gen_range(0..self.cellules_height);
        let col = rng.gen_range(0..self.cellules_width);
        // let neighbors = self.neighbors(row as isize, col as isize);
        let current_idx = self.row_col_as_idx(row as isize, col as isize);
        // alive == spin up +
        // dead == spin down -
        let sigma:f32 = if self.cellules[current_idx].is_alive() { 1.0 } else { -1.0 };
        let mut neighbor_spins = Vec::<f32>::new();
        for cellule in self.neighbors_ising(row as isize, col as isize).iter() {
            neighbor_spins.push(if cellule.is_alive() {1.0 as f32} else {-1.0 as f32});
        };

        // stat mech variables, this is very hacky code that needs refactoring
        let beta: f32 = 1.0/self.temp; // = 1/(k_b*T) 
        let jay: f32 = 1.0;
        let h = (- jay * sigma * (neighbor_spins.iter().sum::<f32>() as f32)) as f32; // the energy

        if h >= 0.0 {
            // if h is geq to zero we should flip the spin with probability one
            if self.cellules[current_idx].is_alive() {
                self.cellules[current_idx].set_dead();
            } else {
                self.cellules[current_idx].set_alive();
            }
        } else {
            // flip with probability exp( 2 * beta * h )
            let p: f32 = rng.gen_range(0.0..1.0);
            if p < (2.0f32 * h * beta).exp() {
                if self.cellules[current_idx].is_alive() {
                    self.cellules[current_idx].set_dead();
                } else {
                    self.cellules[current_idx].set_alive();
                }
            };
        };
    }

    fn neighbors_ising(&self, row: isize, col: isize) -> [Cellule; 4] {
        [
            self.cellules[self.row_col_as_idx(row + 1, col)],
            self.cellules[self.row_col_as_idx(row - 1, col)],
            self.cellules[self.row_col_as_idx(row, col + 1)],
            self.cellules[self.row_col_as_idx(row, col - 1)],
        ]
    }

    fn row_col_as_idx(&self, row: isize, col: isize) -> usize {
        let row = wrap(row, self.cellules_height as isize);
        let col = wrap(col, self.cellules_width as isize);

        row * self.cellules_width + col
    }

    fn view_cellule(&self, idx: usize, cellule: &Cellule, link: &Scope<Self>) -> Html {
        let cellule_status = {
            if cellule.is_alive() {
                "cellule-live"
            } else {
                "cellule-dead"
            }
        };
        html! {
            <div key={idx} class={classes!("game-cellule", cellule_status)}
                onclick={link.callback(move |_| Msg::ToggleCellule(idx))}>
            </div>
        }
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(|_| Msg::Tick);
        let interval = Interval::new(2, move || callback.emit(())); // edit 2 to whatever you want

        // let (cellules_width, cellules_height) = (53, 40);
        // let (cellules_width, cellules_height) = (95, 45);
        let (cellules_width, cellules_height) = (80,80);


        Self {
            active: true, // start with active cellules
            cellules: vec![Cellule::new_dead(); cellules_width * cellules_height],
            cellules_width,
            cellules_height,
            temp: 2.40, // initiate this to 1.0
            _interval: interval,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Random => {
                self.random_mutate();
                log::info!("Random");
                true
            }
            Msg::Start => {
                self.active = true;
                log::info!("Start");
                false
            }
            Msg::Step => {
                // self.step_gol();
                self.step_ising();
                true
            }
            Msg::Reset => {
                self.reset();
                log::info!("Reset");
                true
            }
            Msg::Stop => {
                self.active = false;
                log::info!("Stop");
                false
            }
            Msg::IncreaseT => {
                self.temp += 0.05;
                log::info!("Increase Temperature");
                true
            }
            Msg::DecreaseT => {
                self.temp -= 0.05;
                log::info!("Decrease Temperature");
                true // not to self: what does this do?
            }
            Msg::ToggleCellule(idx) => {
                let cellule = self.cellules.get_mut(idx).unwrap();
                cellule.toggle();
                true
            }
            Msg::Tick => {
                if self.active {
                    // self.step_gol();
                    for _ in 0..10000 { // does 100 steps every tick
                        self.step_ising();
                    }
                    true
                } else {
                    false
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let cell_rows =
            self.cellules
                .chunks(self.cellules_width)
                .enumerate()
                .map(|(y, cellules)| {
                    let idx_offset = y * self.cellules_width;

                    let cells = cellules
                        .iter()
                        .enumerate()
                        .map(|(x, cell)| self.view_cellule(idx_offset + x, cell, ctx.link()));
                    html! {
                        <div key={y} class="game-row">
                            { for cells }
                        </div>
                    }
                });

        html! {
            <div>
                <section class="game-container">
                    <header class="app-header">
                        <img alt="The app logo" src="favicon.ico" class="app-logo"/>
                        <h1 class="app-title">{ "2D Ising Lattice" }</h1>
                    </header>
                    <section class="game-area">
                        <div class="game-of-life">
                            { for cell_rows }
                        </div>
                        <div class="game-buttons">
                            <button class="game-button" onclick={ctx.link().callback(|_| Msg::Random)}>{ "Random" }</button>
                            <button class="game-button" onclick={ctx.link().callback(|_| Msg::Step)}>{ "Step" }</button>
                            <button class="game-button" onclick={ctx.link().callback(|_| Msg::Start)}>{ "Start" }</button>
                            <button class="game-button" onclick={ctx.link().callback(|_| Msg::Stop)}>{ "Stop" }</button>
                            <button class="game-button" onclick={ctx.link().callback(|_| Msg::Reset)}>{ "Reset" }</button>
                            <button class="game-button" onclick={ctx.link().callback(|_| Msg::IncreaseT)}>{ "+T" }</button>
                            <button class="game-button" onclick={ctx.link().callback(|_| Msg::DecreaseT)}>{ "-T" }</button>
                            <br/>
                            <button class="game-button">{ format!("T = {:.2}",self.temp) }</button>
                        </div>
                    </section>
                </section>
                <footer class="app-footer">
                    <strong class="footer-text">
                      { "Rust Ising Models" }
                    </strong>
                    <a href="https://github.com/dcxSt" target="_blank">{ "dcxSt" }</a>
                </footer>
            </div>
        }
    }
}

fn wrap(coord: isize, range: isize) -> usize {
    let result = if coord < 0 {
        coord + range
    } else if coord >= range {
        coord - range
    } else {
        coord
    };
    result as usize
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::trace!("Initializing yew...");
    yew::start_app::<App>();
}
