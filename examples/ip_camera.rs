use tokio;

use hap::{
    accessory::{
        camera::{
            manager::StreamManagerBuilder,
            media::{
                gstreamer::{Gstreamer, StreamConfig},
                MediaProvider,
            },
            protocol::{VIDEO_CODEC_PARAM_LEVEL_TYPES_TYPE3_1, VIDEO_CODEC_PARAM_PROFILE_ID_TYPES_BASELINE},
            CameraAccessory,
        },
        lightbulb::LightbulbAccessory,
        AccessoryCategory, AccessoryInformation,
    },
    server::{IpServer, Server},
    storage::{FileStorage, Storage},
    Config, MacAddress, Pin, Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let mut storage = FileStorage::current_dir().await?;

    let config = match storage.load_config().await {
        Ok(mut config) => {
            config.redetermine_local_ip();
            storage.save_config(&config).await?;
            config
        },
        Err(_) => {
            let config = Config {
                pin: Pin::new([1, 1, 1, 2, 2, 3, 3, 3])?,
                name: "My IP Camera".into(),
                device_id: MacAddress::from([12, 21, 32, 42, 52, 62]),
                category: AccessoryCategory::IpCamera,
                ..Default::default()
            };
            storage.save_config(&config).await?;
            config
        },
    };

    let camera = CameraAccessory::new(
        1,
        1,
        AccessoryInformation {
            name: "My IP Camera".into(),
            ..Default::default()
        },
    )?;
    let video = serde_json::json!({
        "resolutions": [
            [ 1920, 1080, 20 ],
            [ 1024, 768, 20],
            [ 640, 480, 20 ],
        ],
        "codec": {
            "profiles": [
                VIDEO_CODEC_PARAM_PROFILE_ID_TYPES_BASELINE
            ],
            "levels": [
                VIDEO_CODEC_PARAM_LEVEL_TYPES_TYPE3_1
            ]
        }
    });
    let audio = serde_json::json!({
            "codecs": [
            {
                "type": "OPUS",
                "samplerate": 24,
            },
            {
                "type": "AAC-eld",
                "samplerate": 16
            }
        ]
    });
    let stream_config = StreamConfig {
        address: "192.168.1.195".parse().expect("Failed to parse ip address"),
        options: hap::accessory::camera::manager::StreamOptions {
            video,
            audio,
            srtp: true,
        },
        // Change this ip address to whatever your ipcamera ip address is.
        pipeline: "rtspsrc onvif-mode=true location=rtsp://127.0.0.1:8080/h264.sdp ! rtpjitterbuffer ! decodebin"
            .into(),
    };
    let provider = Gstreamer::new(vec![stream_config]).expect("Failed to create gstreamer");

    let server = IpServer::new(config, storage).await?;

    let handle = server.run_handle();
    let _stream_manager = StreamManagerBuilder::new(camera, provider)
        .build(&server)
        .await
        .expect("Failed to build stream manager");

    let _ = handle.await?;

    Ok(())
}
