# A small plugin for [anyrun](https://github.com/Kirottu/anyrun)
Huge thanks to Kirottu for the amazing launcher.

## config
simply write the queries that you want to run in the websearch.ron file:
```
Config (
  prefix_url_map: {
      ":g": "https://google.com/search?q=",
      ":b": "https://search.brave.com/search?q=",
  }
)
```
This file needs to be inside the anyrun config directory

## usage
after running ``` cargo build --release ```, copy the created .so to the plugins directory in the anyrun config.\
``` cp target/release/libanyrun_websearch.so $HOME/.config/anyrun/plugins/. ```\
Then proceed to create the config file for the plugin and add this plugin to the regular anyrun config.
