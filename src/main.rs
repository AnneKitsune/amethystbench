extern crate amethyst;
extern crate time;
#[macro_use]
extern crate log;

use time::precise_time_s;

use amethyst::ecs::*;
use amethyst::prelude::*;
use amethyst::Error;

struct Comp1(f32);
struct Comp2(f32,f32);
struct Comp3(f32,f32,f32);

impl Component for Comp1 {
    type Storage = VecStorage<Self>;
}

impl Component for Comp2 {
    type Storage = VecStorage<Self>;
}

impl Component for Comp3 {
    type Storage = VecStorage<Self>;
}

struct ExampleState {
    ttl: u32,
    last_time: f64,
}

impl<'a, 'b> SimpleState<'a, 'b> for ExampleState {
    fn on_start(&mut self, data: StateData<GameData>) {
        let time_start = precise_time_s();
        self.last_time = time_start;
        for _ in 0..1000000 {
            data.world
                .create_entity()
                .with(Comp1(0.0))
                .build();
            data.world
                .create_entity()
                .with(Comp1(0.0))
                .with(Comp2(0.0,0.0))
                .build();
            data.world
                .create_entity()
                .with(Comp1(0.0))
                .with(Comp2(0.0,0.0))
                .with(Comp3(0.0,0.0,0.0))
                .build();
        }
        info!("Entity creation time: {}", precise_time_s() - time_start);
    }
    fn update(&mut self, _data: &mut StateData<GameData>) -> SimpleTrans<'a,'b> {
        let cur_time = precise_time_s();
        info!("Frame time: {}", cur_time - self.last_time);
        self.last_time = cur_time;
        self.ttl = self.ttl - 1;
        if self.ttl <= 0 {
            Trans::Quit
        } else {
            Trans::None
        }
    }
}

struct Sys1;
struct Sys2;
struct Sys3;

impl<'a> System<'a> for Sys1 {
    type SystemData = ReadStorage<'a,Comp1>;
    fn run(&mut self, comp: Self::SystemData) {
        let mut count = 0;
        for _ in (&comp,).join() {
            count = count + 1;
        }
        assert_eq!(count, 3000000);
    }
}

impl<'a> System<'a> for Sys2 {
    type SystemData = (ReadStorage<'a,Comp1>, ReadStorage<'a,Comp2>);
    fn run(&mut self, (comp,comp2): Self::SystemData) {
        let mut count = 0;
        for (_,_) in (&comp,&comp2).join() {
            count = count + 1;
        }
        assert_eq!(count, 2000000);
    }   
}

impl<'a> System<'a> for Sys3 {
    type SystemData = (ReadStorage<'a,Comp2>, ReadStorage<'a,Comp3>);
    fn run(&mut self, (comp2,comp3): Self::SystemData) {
        let mut count = 0;
        for (_,_) in (&comp2,&comp3).join() {
            count = count + 1;
        }
        assert_eq!(count, 1000000);
    }
}


fn main() -> Result<(), Error> {
    amethyst::start_logger(Default::default());

    let game_data = GameDataBuilder::default()
        .with(Sys1, "sys1", &[])
        .with(Sys2, "sys2", &[])
        .with(Sys3, "sys3", &[]);
    let mut game = Application::build("./", ExampleState{ttl: 2048, last_time: 0.0})?.build(game_data)?;
    game.run();
    Ok(())
}
