#[macro_export]
/// save_on_exit!(T: impl Resource + SaveLoad, FILE_NAME: &str)
macro_rules! save_on_exit {
    ($t:ty, $file_name:expr) => {
        |object: Res<$t>, mut e_app_exit: EventReader<AppExit>| {
            if !e_app_exit.is_empty() {
                e_app_exit.clear();
                if let Err(e) = $crate::save_load::SaveLoad::save_file(&*object, $file_name)
                {
                    warn!("Error saving {}: {}", $file_name, e);
                }
            }
        }
    };
}
