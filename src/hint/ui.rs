use crate::{JpFont, scroll_ui::*, state::RunningStateControlMsg};
use bevy::prelude::*;

const HINT_TEXT: &str = r#"
操作方法:
    切り替えモード:
        Spaceキーで拘束して回転と拘束を解くのを切り替える
    ホールドモード:
        矢印キーを押している間は回転する
    Mキーで入力モードの切り替え
    矢印キーで回転軸の相対座標と回転方向を切り替える
        左:水色軸反時計回転,
        右:ピンク軸時計回転,
        下:水色軸時計回転,
        上:ピンク軸反時計回転
    Rキーでチェックポイントに戻る
    Backspaceキーで起動時の画面に戻る
    Escapeキーで一時停止しヒントを開く

重要な技術:
    歩くようにして直進する
        左右矢印を交互に切り替えることで歩くように移動できる。また、上下矢印を交互に切り替えることで左右のときの逆方向に歩くように移動できる。

"#;

fn inner_text_bundle(font: &Handle<Font>) -> impl Bundle {
    (
        Node {
            max_width: percent(100),
            ..default()
        },
        Text::new(HINT_TEXT),
        TextFont {
            font: font.clone(),
            font_size: 32.0,
            ..default()
        },
        TextLayout::new_with_justify(Justify::Left),
        TextColor::WHITE,
    )
}

#[derive(Component)]
pub struct HintUi;

pub fn spawn_hint_ui(mut commands: Commands, font: Res<JpFont>, mut already_called: Local<bool>) {
    if *already_called {
        return;
    }
    let scroll_ui = spawn_scroll_canvas(&mut commands, 80.0, 100.0);
    let hint_text = commands.spawn(inner_text_bundle(&font.get())).id();
    commands
        .entity(scroll_ui.sub)
        .insert(BackgroundColor(Color::srgb(0.0, 0.0, 0.1)))
        .insert(HintUi)
        .insert(Visibility::Hidden)
        .add_child(hint_text);
    *already_called = true;
}

pub fn hint_ui_show(
    mut msg: MessageReader<RunningStateControlMsg>,
    mut ui_que: Query<&mut Visibility, With<HintUi>>,
) {
    for state in msg.read() {
        for mut visibility in &mut ui_que {
            *visibility = match state {
                RunningStateControlMsg::Pause => Visibility::Visible,
                RunningStateControlMsg::Resume => Visibility::Hidden,
            }
        }
    }
}
