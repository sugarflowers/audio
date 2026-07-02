use kira::{
    manager::{AudioManager, AudioManagerSettings, backend::DefaultBackend},
    sound::{
        static_sound::{StaticSoundData},
        PlaybackState,
    },
};
use std::{
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

/// グローバル AudioManager
static mut MANAGER: Option<Arc<Mutex<AudioManager<DefaultBackend>>>> = None;

/// 再生スレッドを保持する（プログラム終了防止）
static mut PLAY_THREAD: Option<JoinHandle<()>> = None;

/// 初期化
pub fn init_audio_manager() {
    unsafe {
        if MANAGER.is_none() {
            MANAGER = Some(Arc::new(Mutex::new(
                AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap(),
            )));
        }
    }
}

pub struct Audio;

impl Audio {
    pub fn new() -> Self {
        init_audio_manager();
        Audio
    }

    /// 非ブロッキング再生（メイン処理は止まらない）
    /// ただしプログラム終了時に再生が終わるまで終了しない
    pub fn play(&self, file_path: &str) {
        let data = StaticSoundData::from_file(file_path).unwrap();
        let manager = unsafe { MANAGER.as_ref().unwrap().clone() };
        let handle = manager.lock().unwrap().play(data).unwrap();

        // 再生終了まで待つスレッド（非ブロッキング）
        let t = thread::spawn(move || {
            while handle.state() != PlaybackState::Stopped {
                thread::sleep(std::time::Duration::from_millis(10));
            }
        });

        // スレッドを保持しておく（プログラム終了防止）
        unsafe {
            PLAY_THREAD = Some(t);
        }
    }
}

/// プログラム終了時に呼ぶ（再生が終わるまで終了しない）
pub fn wait_for_audio() {
    unsafe {
        if let Some(t) = PLAY_THREAD.take() {
            let _ = t.join();
        }
    }
}
