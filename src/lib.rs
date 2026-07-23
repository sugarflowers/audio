use kira::{
    manager::{AudioManager, AudioManagerSettings, backend::DefaultBackend},
    sound::{
        static_sound::StaticSoundData,
        PlaybackState,
    },
};
use std::{
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

/// グローバル AudioManager
static mut MANAGER: Option<Arc<Mutex<AudioManager<DefaultBackend>>>> = None;

/// 再生ワーカーの JoinHandle を保持する
static mut WORKERS: Vec<JoinHandle<()>> = Vec::new();

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

    // 非ブロッキング再生ワーカーを作成
    pub fn play(&self, file_path: &str) {
        let data = StaticSoundData::from_file(file_path).unwrap();
        let manager = unsafe { MANAGER.as_ref().unwrap().clone() };
        let handle = manager.lock().unwrap().play(data).unwrap();

        // 再生ワーカー（非ブロッキング）
        let worker = thread::spawn(move || {
            while handle.state() != PlaybackState::Stopped {
                thread::sleep(std::time::Duration::from_millis(10));
            }
            // 再生終了 → スレッドは自動で破棄される
        });

        // ワーカーを保持（プログラム終了防止）
        unsafe {
            WORKERS.push(worker);
        }
    }
}

/// プログラム終了時に呼ぶ（全ワーカーが終わるまで終了しない）
pub fn wait_for_all_audio() {
    unsafe {
        for worker in WORKERS.drain(..) {
            let _ = worker.join();
        }
    }
}
