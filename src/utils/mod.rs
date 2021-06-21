use crate::config::ConfigurationData;
use std::{ fs::File, io::Read };

pub fn read_config(file: &str) -> ConfigurationData {
   let mut file = File::open(file).expect("Could not read configuration file.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let asdf: ConfigurationData = serde_yaml::from_str(&contents).expect("Didnt work");
    asdf
}



/// The user agent used for the reqwest client.
pub const REQWEST_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

pub const TITLE: &str = " $$$$$$\\
$$  __$$\\
$$ /  \\__| $$$$$$\\   $$$$$$\\  $$\\   $$\\
\\$$$$$$\\   \\____$$\\ $$  __$$\\ $$ |  $$ |
 \\____$$\\  $$$$$$$ |$$ |  \\__|$$ |  $$ |
$$\\   $$ |$$  __$$ |$$ |      $$ |  $$ |
\\$$$$$$  |\\$$$$$$$ |$$ |      \\$$$$$$  |
 \\______/  \\_______|\\__|       \\______/";