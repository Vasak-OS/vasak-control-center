use alsa::{
    mixer::{Selem, SelemId},
    Mixer,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct VolumeInfo {
    current: i64,
    min: i64,
    max: i64,
    is_muted: bool,
}

fn with_mixer<F, T>(f: F) -> Result<T, String>
where
    F: FnOnce(&Mixer, &Selem) -> Result<T, String>,
{
    let mixer = Mixer::new("default", false).map_err(|e| e.to_string())?;
    let selem_id = SelemId::new("Master", 0);
    let selem = mixer
        .find_selem(&selem_id)
        .ok_or("No se encontró el control Master")?;
    f(&mixer, &selem)
}

#[tauri::command]
pub fn get_volume() -> Result<VolumeInfo, String> {
    with_mixer(|_mixer, selem| {
        let (min, max) = selem.get_playback_volume_range();
        let current = selem
            .get_playback_volume(alsa::mixer::SelemChannelId::mono())
            .map_err(|e| e.to_string())?;

        let is_muted = selem
            .get_playback_switch(alsa::mixer::SelemChannelId::mono())
            .map_err(|e| e.to_string())?
            == 0;

        Ok(VolumeInfo {
            current,
            min,
            max,
            is_muted,
        })
    })
}

#[tauri::command]
pub fn set_volume(volume: i64) -> Result<(), String> {
    with_mixer(|_mixer, selem| {
        selem
            .set_playback_volume_all(volume)
            .map_err(|e| e.to_string())
    })
}

#[tauri::command]
pub fn toggle_mute() -> Result<bool, String> {
    with_mixer(|_mixer, selem| {
        let current = selem
            .get_playback_switch(alsa::mixer::SelemChannelId::mono())
            .map_err(|e| e.to_string())?;

        let new_state = if current == 1 { 0 } else { 1 };
        selem
            .set_playback_switch_all(new_state)
            .map_err(|e| e.to_string())?;

        Ok(new_state == 1)
    })
}
