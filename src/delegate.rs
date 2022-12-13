use druid::{commands, AppDelegate, Handled};

use crate::data::ViewImageData;

pub struct ViewImageDelegate;

impl AppDelegate<ViewImageData> for ViewImageDelegate {
    fn command(
        &mut self,
        _ctx: &mut druid::DelegateCtx,
        _target: druid::Target,
        cmd: &druid::Command,
        data: &mut ViewImageData,
        _env: &druid::Env,
    ) -> druid::Handled {
        if let Some(file) = cmd.get(commands::OPEN_FILE) {
            let path = file.path();
            let is_file = path.is_file();

            if is_file {
                if let Some(file) = path.to_str() {
                    println!("File is : {}", file.to_string());
                    data.url_image = Some(file.to_string());
                }
            }
            return Handled::Yes;
        }

        Handled::No
    }
}
