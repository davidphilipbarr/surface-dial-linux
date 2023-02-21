use crate::controller::{ControlMode, ControlModeMeta};
use crate::dial_device::DialHaptics;
use crate::error::{Error, Result};
use crate::fake_input;

use evdev_rs::enums::EV_KEY;

pub struct ZoomSVG {}

impl ZoomSVG {
    pub fn new() -> ZoomSVG {
        ZoomSVG {}
    }
}

impl ControlMode for ZoomSVG {
    fn meta(&self) -> ControlModeMeta {
        ControlModeMeta {
            name: "ZoomSVG",
            icon: "edit-find",
            haptics: true,
            steps: 36,
        }
    }

    fn on_btn_press(&mut self, _: &DialHaptics) -> Result<()> {
        Ok(())
    }

    fn on_btn_release(&mut self, _haptics: &DialHaptics) -> Result<()> {
        Ok(())
    }

    fn on_dial(&mut self, _: &DialHaptics, delta: i32) -> Result<()> {
        if delta > 0 {
            eprintln!("zoom in");
            fake_input::key_click(&[EV_KEY::KEY_EQUAL])
                .map_err(Error::Evdev)?;
        } else {
            eprintln!("zoom out");
            fake_input::key_click(&[EV_KEY::KEY_MINUS])
                .map_err(Error::Evdev)?;
        }

        Ok(())
    }
}
