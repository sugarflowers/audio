use kira::{
    manager::{AudioManager, AudioManagerSettings, backend::DefaultBackend},
    sound::{
        static_sound::{StaticSoundData, StaticSoundHandle},
        PlaybackState,
    },
};
use std::{
    sync::mpsc::{Sender, Receiver, channel},
    thread,
};

/// 再生コマンド
pub enum AudioCommand {
    Play(String),
}

/// AudioManager を永続スレッドで動かす
pub fn start_audio_thread() -> Sender<AudioCommand> {
    let (tx, rx): (Sender<AudioCommand>, Receiver<AudioCommand>) = channel();

    thread::spawn(move || {
        let mut manager =
            AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap();

        while let Ok(cmd) = rx.recv() {
            match cmd {
                AudioCommand::Play(path) => {
                    let data = StaticSoundData::from_file(path).unwrap();
                    let handle = manager.play(data).unwrap();

                    // 再生終了まで待つ専用スレッド
                    thread::spawn(move || {
                        while handle.state() != PlaybackState::Stopped {
                            thread::sleep(std::time::Duration::from_millis(10));
                        }
                    });
                }
            }
        }
    });

    tx
}

/// Audio は「再生要求を送るだけ」
pub struct Audio {
    tx: Sender<AudioCommand>,
}

impl Audio {
    pub fn new(tx: Sender<AudioCommand>) -> Self {
        Self { tx }
    }

    pub fn play(&self, file_path: &str) {
        let _ = self.tx.send(AudioCommand::Play(file_path.to_string()));
    }
}
