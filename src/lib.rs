use kira::{
    manager::{AudioManager, AudioManagerSettings, backend::DefaultBackend},
    sound::{
        static_sound::{StaticSoundData, StaticSoundHandle},
        PlaybackState,
    },
};
use std::{
    sync::{Arc, Mutex},
    thread,
};

static MANAGER: once_cell::sync::Lazy<Arc<Mutex<AudioManager<DefaultBackend>>>> =
    once_cell::sync::Lazy::new(|| {
        Arc::new(Mutex::new(
            AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap(),
        ))
    });

pub struct Audio;

impl Audio {
    pub fn new() -> Self {
        Audio
    }

    /// 再生が終わるまでプログラムを終了しない
    pub fn play_blocking(&self, file_path: &str) {
        let data = StaticSoundData::from_file(file_path).unwrap();

        // 再生開始
        let handle = MANAGER.lock().unwrap().play(data).unwrap();

        // 再生終了まで待つ専用スレッド
        let t = thread::spawn(move || {
            while handle.state() != PlaybackState::Stopped {
                thread::sleep(std::time::Duration::from_millis(10));
            }
        });

        // 再生終了まで待つ
        let _ = t.join();
    }
}
