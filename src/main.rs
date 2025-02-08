use bevy::{prelude::*, winit::WinitSettings};
use controlled_chaos::*;

fn main() -> AppExit {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(ToggleButtonPlugin::<ApuMaster>::default())
    .insert_resource(WinitSettings::desktop_app())
    .add_systems(Startup, startup)
    .add_systems(Update, log_apu_master_changes)
    .run()
}

fn startup(mut commands: Commands) {
  commands.spawn((Camera2d, IsDefaultUiCamera));

  let ui_root = commands
    .spawn(Node {
      width: Val::Percent(100.0),
      height: Val::Percent(100.0),
      align_items: AlignItems::Start,
      justify_content: JustifyContent::Start,
      ..Default::default()
    })
    .id();

  commands.queue(SpawnToggleButton::<ApuMaster> {
    parent: Some(ui_root),
    text: Text::new("APU MASTER"),
    ..Default::default()
  });
}

#[derive(Debug, Default, Component)]
struct ApuMaster(bool);

impl Toggle for ApuMaster {
  #[inline]
  fn is_toggled(&self) -> bool {
    self.0
  }

  #[inline]
  fn is_fault(&self) -> bool {
    false
  }

  #[inline]
  fn toggle(&mut self) {
    self.0 = !self.0
  }
}

fn log_apu_master_changes(
  query: Option<Single<&ApuMaster, Changed<ApuMaster>>>,
) {
  let Some(apu_master) = query.map(Single::into_inner) else {
    return;
  };
  info!("APU Master changed: {}", apu_master.is_toggled());
}
