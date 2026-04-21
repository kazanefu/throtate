use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    bevy::asset::embedded_asset!(app, "fonts/NotoSansJP-Bold.ttf"); 
    app.run(); 
    println!("Hello, world!");
}
