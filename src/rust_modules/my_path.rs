use std::path::PathBuf;

pub fn get_dir_path() -> PathBuf {
    let option_home_dir = dirs::home_dir();
    match option_home_dir {
      Some(mut dir) => {
        dir.push("Development");
        dir.push("rust");
        dir.push("air_quality");
        dir.push("json_files");
        dir.push("json_files");
        dir.set_file_name("airQuality");
        dir.set_extension("json");
  
        dir
      },
      None => {
        let mut path: PathBuf = [
          r"/",
          "home",
          "tonatiuh.martinez",
          "Development",
          "rust",
          "air_quality",
        ].iter().collect();
        path.push("json_files");
        path.set_file_name("airQuality");
        path.set_extension("json");
  
        path
      }
    }
  }