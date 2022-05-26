extern crate caper;

use caper::game::*;
use caper::imgui::Ui;
use caper::input::Key;
use caper::mesh::gen_cube;
use caper::types::{DefaultTag, RenderItemBuilder, TransformBuilder};
use caper::utils::handle_fp_inputs;

fn main() {
    // crate an instance of the game struct
    let (mut game, event_loop) = Game::<DefaultTag>::new();

    // define some items to be rendered
    game.add_render_item(
        RenderItemBuilder::default()
            .vertices(gen_cube())
            .instance_transforms(vec![TransformBuilder::default()
                .pos((-0.5, 0.0, -5.0))
                .build()
                .unwrap()])
            .build()
            .unwrap(),
    );

    // run the engine update
    start_loop(event_loop, move |events| {
        game.update(
            |_: &Ui| {},
            |g: &mut Game<DefaultTag>| -> UpdateStatus {
                // update the first person inputs
                handle_fp_inputs(&mut g.input, &mut g.cams[0]);

                // quit
                if g.input.keys_down.contains(&Key::Escape) {
                    return UpdateStatus::Finish;
                }

                UpdateStatus::Continue
            },
            events,
        )
    });
}