// This is a rust version of example.c
// https://www.fluidsynth.org/api/example_8c-example.html

use std::{thread, time};
use rand::distributions::uniform::UniformDuration;
use rand::{self, thread_rng, Rng};

use fluidsynth::settings::Settings;
use fluidsynth::synth::Synth;
use fluidsynth::audio::AudioDriver;

fn main() {
    // Create the settings.
    let mut settings = Settings::new();
    
    // Change the settings if necessary
    settings.setint("midi.autoconnect", 1);
    
    // Create the synthesizer.
    let mut synth = Synth::new(&mut settings);

    // Load a SoundFont and reset presets.
    let _sfont_id = synth.sfload("example.sf2", 1);

    // Create the audio driver.
    let _adriver = AudioDriver::new(&mut settings, &mut synth);

    for _i in 0..12 {
        let key: i32 = thread_rng().gen_range(0..12) + 60;
        synth.noteon(0, key, 80);
        let duration = time::Duration::from_millis(1000);
        thread::sleep(duration);
        synth.noteoff(0, key);
    }
}
