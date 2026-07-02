use kira::{
    manager::{AudioManager, AudioManagerSettings, backend::DefaultBackend},
    sound::static_sound::StaticSoundData,
};
use std::{
    sync::{mpsc::{Sender, Receiver, channel}, Arc},
    thread,
};

/// 再生コマンド
enum AudioCommand {
    Play(String),
}

/// AudioManager を永続スレッドで動かす
fn start_audio_thread() -> Sender<AudioCommand> {
    let (tx, rx): (Sender<AudioCommand>, Receiver<AudioCommand>) = channel();

    thread::spawn(move || {
        let mut manager =
            AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap();

        // このスレッドが生存している限り再生は止まらない
        while let Ok(cmd) = rx.recv() {
            match cmd {
                AudioCommand::Play(path) => {
                    let data = StaticSoundData::from_file(path).unwrap();
                    manager.play(data).unwrap();
                }
            }
        }
    });

    tx
}

/// Audio は単なる「再生要求を送るだけ」の構造にする
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
