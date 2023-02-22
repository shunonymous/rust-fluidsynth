// This is a rust version of fluidsynth_simple.c
// https://www.fluidsynth.org/api/fluidsynth_simple_8c-example.html

use std::{env, io, process};

use fluidsynth::settings::Settings;
use fluidsynth::synth::Synth;
use fluidsynth::audio::AudioDriver;

fn main() {
    let argv: Vec<String> = env::args().collect();
    if argv.len() < 2 {
	println!("Usage: fluidsynth_simple [soundfont]");
	process::exit(1);
    }

    let mut settings = Settings::new();
    let mut synth = Synth::new(&mut settings);
    synth.sfload(&argv[1], 1);

    let _adriver = AudioDriver::new(&mut settings, &mut synth);

    synth.noteon(0, 60, 100);

    let mut _str = String::new();
    
    println!("Press \"Enter\" to stop: ");
    io::stdin().read_line(&mut _str);
    println!("done");
}
