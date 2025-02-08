//! This example illustrates how to create a button that changes color and text based on its
//! interaction state.

use bevy::{
  color::palettes::basic::*, prelude::*, utils::tracing, winit::WinitSettings,
};

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
    .insert_resource(WinitSettings::desktop_app())
    .add_systems(Startup, setup)
    .add_systems(Update, (button_system, apu_system, apu_color).chain())
    .run();
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

#[derive(Debug, Clone, Copy, Component, Hash, Default)]
struct APUMaster(bool);

fn button_system(
  mut interaction_query: Query<
    (
      &Interaction,
      &mut BackgroundColor,
      &mut BorderColor,
      &Children,
    ),
    (Changed<Interaction>, With<Button>),
  >,
  mut text_query: Query<&mut Text>,
) {
  for (interaction, mut color, mut border_color, children) in
    &mut interaction_query
  {
    let mut text = text_query.get_mut(children[0]).unwrap();
    match *interaction {
      Interaction::Pressed => {
        *color = PRESSED_BUTTON.into();
        border_color.0 = RED.into();
      }
      Interaction::Hovered => {
        *color = HOVERED_BUTTON.into();
        border_color.0 = Color::WHITE;
      }
      Interaction::None => {
        *color = NORMAL_BUTTON.into();
        border_color.0 = Color::BLACK;
      }
    }
  }
}

fn apu_system(
  mut interaction_query: Query<
    &Interaction,
    (Changed<Interaction>, With<Button>),
  >,
  mut apu_query: Query<&mut APUMaster>,
) {
  for (interaction) in &mut interaction_query {
    let mut apu_master = apu_query.single_mut();
    if *interaction == Interaction::Pressed {
      apu_master.0 = !apu_master.0;
    }
  }
}

fn apu_color(
  button_query: Single<(&mut BackgroundColor, &mut BorderColor), With<Button>>,
  apu_query: Query<&APUMaster, Changed<APUMaster>>,
) {
  tracing::info!("APU color");
  let apu_query = apu_query.single();
  let (mut color, mut border_color) = button_query.into_inner();
  match apu_query.0 {
    true => border_color.0 = Color::linear_rgb(0.0, 0.9, 0.0),
    false => border_color.0 = Color::BLACK,
  }
}

fn setup(mut commands: Commands) {
  // ui camera
  commands.spawn(Camera2d);
  commands
    .spawn((
      Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..default()
      },
      APUMaster(false),
    ))
    .with_children(|parent| {
      parent
        .spawn((
          Button,
          Node {
            width: Val::Px(150.0),
            height: Val::Px(65.0),
            border: UiRect::all(Val::Px(5.0)),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            ..default()
          },
          BorderColor(Color::BLACK),
          BorderRadius::MAX,
          BackgroundColor(NORMAL_BUTTON),
        ))
        .with_child((
          Text::new("APU\nMaster"),
          TextFont {
            font_size: 33.0,
            ..default()
          },
          TextColor(Color::srgb(0.9, 0.9, 0.9)),
        ));
    });
}
