/*
Copyright © 2023 Fabio Lenherr

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program. If not, see <http://www.gnu.org/licenses/>.
*/

use abi_stable::std_types::{ROption, RString, RVec};
use anyrun_plugin::{anyrun_interface::HandleResult, plugin, Match, PluginInfo};
use serde::Deserialize;
use std::{collections::HashMap, fs};

#[derive(Deserialize, Debug, Clone)]
struct Config {
    prefix_url_map: HashMap<String, String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            prefix_url_map: HashMap::new(),
        }
    }
}

fn init(config_dir: RString) -> Config {
    match fs::read_to_string(format!("{}/websearch.ron", config_dir)) {
        Ok(content) => ron::from_str(&content).unwrap_or_default(),
        Err(_) => Config::default(),
    }
}

fn info() -> PluginInfo {
    PluginInfo {
        name: "Websearch".into(),
        icon: "edit-find-symbolic".into(),
    }
}

fn get_matches(input: RString, config: &mut Config) -> RVec<Match> {
    for (prefix, url) in config.prefix_url_map.iter() {
        let prefix_with_delim = format!("{}", prefix);
        if input.starts_with(&prefix_with_delim) {
            let (_, command) = input.split_once(&prefix_with_delim).unwrap();
            if !command.is_empty() {
                return vec![Match {
                    title: (url.clone() + command).into(),
                    description: ROption::RSome(RString::from("websearch")),
                    use_pango: false,
                    icon: ROption::RNone,
                    id: ROption::RNone,
                }]
                .into();
            } else {
                return RVec::new();
            }
        }
    }
    RVec::new()
}

fn handler(selection: Match, _config: &mut Config) -> HandleResult {
    let search_string = selection.title.as_str();
    let result = open::that(search_string);
    if result.is_err() {
        panic!("failed to open default browser");
    }
    HandleResult::Close
}

plugin!(init, info, get_matches, handler, Config);
