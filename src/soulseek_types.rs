// soulseek types for dbus communication with server

use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, zbus::zvariant::Type,
)]
pub enum ConnectionState {
    #[default]
    Disconnected,
    Connecting,
    Connected,
    LoggingIn,
    Ready,
    Error,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, zbus::zvariant::Type,
)]
pub enum DownloadState {
    #[default]
    Pending,
    Connecting,
    Downloading,
    Complete,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, zbus::zvariant::Type)]
pub struct SharedFile {
    pub filename: String,
    pub size: u64,
    pub bitrate: u32,
    pub length_secs: u64,
    pub vbr: bool,
    pub sample_rate: u32,
}

impl SharedFile {
    pub fn extension(&self) -> Option<&str> {
        self.filename.rsplit('.').next()
    }

    pub fn basename(&self) -> &str {
        self.filename
            .rsplit(['/', '\\'])
            .next()
            .unwrap_or(&self.filename)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, zbus::zvariant::Type)]
pub struct RankedFile {
    pub file: SharedFile,
    pub username: String,
    pub free_slots: bool,
    pub upload_speed: u32,
    pub queue_size: u32,
    pub quality_score: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, zbus::zvariant::Type)]
pub struct Download {
    pub id: u32,
    pub username: String,
    pub remote_path: String,
    pub local_path: std::path::PathBuf,
    pub size: u64,
    pub downloaded: u64,
    pub state: DownloadState,
    pub quality_score: u32,
    pub display_name: String,
    pub token: u32,
}

impl Download {
    pub fn progress(&self) -> f64 {
        if self.size == 0 {
            0.0
        } else {
            self.downloaded as f64 / self.size as f64
        }
    }

    pub fn progress_percent(&self) -> String {
        format!("{:.1}%", self.progress() * 100.0)
    }
}
