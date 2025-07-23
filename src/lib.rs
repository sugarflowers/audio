use kira::{
	manager::{
		AudioManager, AudioManagerSettings,
		backend::DefaultBackend,
	},
	sound::static_sound::{
        StaticSoundData, StaticSoundHandle,
    },
    tween::Tween,
};

use std::{
    thread,
    sync::{
        Arc, Mutex,
    },
    default::Default,
};

pub struct Audio {
    playing: Arc<Mutex<bool>>,
    manager: Arc<Mutex<AudioManager>>,
    sound: Arc<Mutex<Option<StaticSoundHandle>>>,
}

impl Audio {
    pub fn new() -> Self {
        let manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap();
        Self {
            playing: Arc::new(Mutex::new(false)),
            manager: Arc::new(Mutex::new(manager)),
            sound: Arc::new(Mutex::new(None)),
        }
    }

    pub fn play(&mut self, file_path: &str) {
        if *self.playing.lock().unwrap() {
            self.stop();
        }
        let static_sound_data = StaticSoundData::from_file(file_path).unwrap();
        let manager = Arc::clone(&self.manager);
        let playing = Arc::clone(&self.playing);

        *self.sound.lock().unwrap() = Some(self.manager.lock().unwrap().play(static_sound_data.clone()).unwrap());
        *self.playing.lock().unwrap() = true;

        thread::spawn(move || {
            let mgr = manager.lock().unwrap();
            while mgr.num_sounds() > 0 && *playing.lock().unwrap() {};
            *playing.lock().unwrap() = false;
        });

    }

    pub fn stop(&mut self) {
        let snd = Arc::clone(&self.sound);
        let playing = Arc::clone(&self.playing);

        if *playing.lock().unwrap() {
            if let Some(ref mut handle) = *snd.lock().unwrap() {
                handle.stop(Tween {..Default::default()});
            }
            *playing.lock().unwrap() = false;
        }
    }
}

impl Default for Audio {
    fn default() -> Self {
        Self::new()
    }
}

