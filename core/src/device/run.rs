use crate::Command;
use super::DeviceCommandState;
use async_trait::async_trait;

use clap::Clap;

#[derive(Clap)]
pub struct RunDeviceCommand {}

#[async_trait(?Send)]
impl Command<DeviceCommandState> for RunDeviceCommand {
    async fn run(&self, state: DeviceCommandState) -> anyhow::Result<()> {
        log::info!("Starting virtual device with ID: {}", state.config.device_id);
        device::run(device::Config {
            device_id: state.config.device_id,
            device_password: state.config.device_password,
            lighthouse: device::LighthouseConfig {
                host: state.config.lighthouse.host,
                port: state.config.lighthouse.port,
            },
        }).await
    }
}
