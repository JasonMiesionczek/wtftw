extern crate serialize;

use std::os::homedir;
use std::sync::RWLock;
use serialize::{Encodable, Decodable, json, Decoder};
use std::io::{File, Open, ReadWrite, Reader};

/// Common configuration options for the window manager.
#[deriving(Encodable,Decodable,Clone)]
pub struct Config {
    /// Whether focus follows mouse movements or
    /// only click events and keyboard movements.
    pub focus_follows_mouse: bool,
    /// Border color for focused windows.
    pub focus_border_color: u32,
    /// Border color for unfocused windows.
    pub border_color: u32,
    /// Border width. This is the same for both, focused and unfocused.
    pub border_width: u32,
    /// Default spacing between windows
    pub spacing: u32,
    /// Default terminal to start
    pub terminal: (String, String),
    /// Keybind for the terminal
    pub terminal_key: String,
    /// Path to the logfile
    pub logfile: String,
    /// Default tags for workspaces
    pub tags: Vec<String>,
    /// Default launcher application
    pub launcher: String,
    /// Keybind for the launcher and configuration reloading
    pub launch_key: String
}

/// Will pass around RWLock<ConfigLock>, deref to Config, and be updatable for conf
pub struct ConfigLock {
    conf: RWLock<Config>
}

/// Allow existing code to use the ConfigLock
impl ConfigLock {
    pub fn current(&self) -> Config {
        self.conf.read().deref().clone()
    }
}


/// Trait for allowing of overwriting of the RWLocked Config struct
trait Updatable{
    /// Updates the configuration globally to achieve config
    /// hotswapping.
    fn update(&mut self,new_conf: Config);
}

impl Updatable for ConfigLock {
    fn update(&mut self,new_conf: Config) {
        self.conf = RWLock::new(new_conf);
    }
}

impl Config {
    /// Create the Config from a json file
    pub fn initialize() -> ConfigLock {
        //Default version of the config, for fallback
        let conf = Config {
                    focus_follows_mouse: true,
                    focus_border_color:  0x00B6FFB0,
                    border_color:        0x00FFB6B0,
                    border_width:        2,
                    spacing:             10,
                    terminal:            (String::from_str("xterm"), String::from_str("-fg White -bg Black")),
                    terminal_key:        String::from_str("Return"),
                    logfile:             format!("{}/.wtftw.log", homedir().unwrap().to_c_str()),
                    tags:                vec!(
                                            String::from_str("1: term"),
                                            String::from_str("2: web"),
                                            String::from_str("3: code"),
                                            String::from_str("4 media")),
                    launcher:            String::from_str("gmrun"),
                    launch_key:          String::from_str("p")
        };

        let path = Path::new(format!("{}/.wtftwrc", homedir().unwrap().to_c_str()));
        let mut conf_file = File::open_mode(&path, Open, ReadWrite).unwrap();
        let dec_conf = match json::decode(conf_file.read_to_string().unwrap().as_slice()) {
            Ok(v) => v,
            Err(_) =>{
                        info!("config file is corrupted. falling back to default.");
                        //Let's just roll back to the default
                        //conf_file.truncate(0);
                        conf_file.write_str(json::encode::<Config>(&conf).as_slice());
                        conf_file.fsync();
                        conf
                     }
        };

        ConfigLock {
            conf: RWLock::new(dec_conf)
        }
    }
}
