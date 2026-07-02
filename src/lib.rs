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

/// AudioManager をグローバルに保持する
/// once_cell を使わずに Option + Mutex で管理する
static mut MANAGER: Option<Arc<Mutex<AudioManager<DefaultBackend>>>> = None;

/// 初期化（lib.rs 内で一度だけ呼ぶ）
pub fn init_audio_manager() {
    unsafe {
        if MANAGER.is_none() {
            MANAGER = Some(Arc::new(Mutex::new(
                AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap(),
            )));
        }
    }
}

/// Audio 構造体
pub struct Audio;

impl Audio {
    pub fn new() -> Self {
        init_audio_manager();
        Audio
    }

    /// 音声が終わるまでプログラムを終了しない
    pub fn play_blocking(&self, file_path: &str) {
        let data = StaticSoundData::from_file(file_path).unwrap();

        let manager = unsafe { MANAGER.as_ref().unwrap().clone() };
        let handle = manager.lock().unwrap().play(data).unwrap();

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
