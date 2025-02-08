use core::marker::PhantomData;

use bevy::prelude::*;

pub struct ToggleButtonPlugin<T>(PhantomData<fn() -> T>)
where
  T: Component + Toggle + Default;

impl<T> Default for ToggleButtonPlugin<T>
where
  T: Component + Toggle + Default,
{
  #[inline]
  fn default() -> Self {
    Self(PhantomData)
  }
}

impl<T> Plugin for ToggleButtonPlugin<T>
where
  T: Component + Toggle + Default,
{
  fn build(&self, app: &mut App) {
    _ = app.try_register_required_components::<T, Button>();

    app.init_resource::<ToggleButtonAssets>().add_systems(
      PostUpdate,
      (handle_toggle_button_interactions::<T>, sync_toggle_ui::<T>).chain(),
    );
  }
}

#[derive(Clone, Resource)]
pub struct ToggleButtonAssets {
  pub size: Val,
  pub border_radius: Val,
  pub text_font: TextFont,
  pub background_color: Color,
  pub fault_color: Color,
  pub color: Color,
}

impl Default for ToggleButtonAssets {
  #[inline]
  fn default() -> Self {
    Self {
      size: Val::Px(100.0),
      border_radius: Val::Px(3.0),
      text_font: TextFont {
        font_size: 15.0,
        ..Default::default()
      },
      background_color: Color::srgb_u8(0x1b, 0x1b, 0x1b),
      fault_color: Color::srgb_u8(0xf2, 0x83, 0x49),
      color: Color::srgb_u8(0x1a, 0xe1, 0x90),
    }
  }
}

#[derive(Component)]
pub struct ToggleFault;

#[derive(Component)]
pub struct ToggleOn;

#[derive(Default)]
pub struct SpawnToggleButton<T>
where
  T: Component + Toggle + Default,
{
  pub parent: Option<Entity>,
  pub text: Text,
  pub toggle: T,
}

impl<T> Command for SpawnToggleButton<T>
where
  T: Component + Toggle + Default,
{
  fn apply(self, world: &mut World) {
    let toggle_button_assets = world.resource::<ToggleButtonAssets>().clone();
    let toggle = T::default();
    let is_toggled = toggle.is_toggled();
    let is_fault = toggle.is_fault();

    let entity = world
      .spawn((Node {
        width: toggle_button_assets.size,
        flex_direction: FlexDirection::Column,
        column_gap: toggle_button_assets.border_radius,
        ..Default::default()
      },))
      .with_children(|commands| {
        commands.spawn((
          self.text,
          toggle_button_assets.text_font,
          TextLayout {
            justify: JustifyText::Center,
            linebreak: LineBreak::WordBoundary,
          },
        ));

        commands
          .spawn((
            toggle,
            Node {
              width: toggle_button_assets.size,
              height: toggle_button_assets.size,
              padding: UiRect::all(toggle_button_assets.border_radius),
              flex_direction: FlexDirection::Column,
              column_gap: toggle_button_assets.border_radius,
              justify_content: JustifyContent::SpaceEvenly,
              justify_items: JustifyItems::Center,
              align_items: AlignItems::Center,
              ..Default::default()
            },
            BackgroundColor(toggle_button_assets.background_color),
            BorderRadius::all(toggle_button_assets.border_radius),
          ))
          .with_children(|commands| {
            commands
              .spawn((
                if is_fault {
                  Visibility::Inherited
                } else {
                  Visibility::Hidden
                },
                ToggleFault,
                Node {
                  justify_items: JustifyItems::Center,
                  align_items: AlignItems::Center,
                  border: UiRect::all(toggle_button_assets.border_radius),
                  ..Default::default()
                },
              ))
              .with_child((
                Text::new("FAULT"),
                TextColor(toggle_button_assets.fault_color),
              ));

            commands
              .spawn((
                if is_toggled {
                  Visibility::Inherited
                } else {
                  Visibility::Hidden
                },
                ToggleOn,
                Node {
                  padding: UiRect::axes(Val::Percent(25.0), Val::Percent(5.0)),
                  justify_items: JustifyItems::Center,
                  align_items: AlignItems::Center,
                  border: UiRect::all(toggle_button_assets.border_radius),
                  ..Default::default()
                },
                BorderRadius::all(toggle_button_assets.border_radius),
                BorderColor(toggle_button_assets.color),
              ))
              .with_child((
                Text::new("ON"),
                TextColor(toggle_button_assets.color),
              ));
          });
      })
      .id();

    if let Some(parent) = self.parent {
      world.entity_mut(parent).add_child(entity);
    }
  }
}

pub trait Toggle {
  fn is_toggled(&self) -> bool;
  fn is_fault(&self) -> bool;
  fn toggle(&mut self);
}

pub fn handle_toggle_button_interactions<T>(
  mut toggle_query: Query<(&Interaction, &mut T), Changed<Interaction>>,
) where
  T: Component + Toggle,
{
  for (interaction, mut toggle) in toggle_query.iter_mut() {
    if *interaction == Interaction::Pressed {
      toggle.toggle();
    }
  }
}

pub fn sync_toggle_ui<T>(
  toggle_query: Query<
    (&Children, &T),
    (Changed<T>, Without<ToggleOn>, Without<ToggleFault>),
  >,
  mut toggle_on_query: Query<
    &mut Visibility,
    (With<ToggleOn>, Without<T>, Without<ToggleFault>),
  >,
  mut toggle_fault_query: Query<
    &mut Visibility,
    (With<ToggleFault>, Without<T>, Without<ToggleFault>),
  >,
) where
  T: Component + Toggle,
{
  for (children, toggle) in toggle_query.iter() {
    for &child in children {
      if let Ok(mut visibility) = toggle_on_query.get_mut(child) {
        *visibility = match toggle.is_toggled() {
          false => Visibility::Hidden,
          true => Visibility::Inherited,
        };
      }
    }

    for &child in children {
      if let Ok(mut visibility) = toggle_fault_query.get_mut(child) {
        *visibility = match toggle.is_fault() {
          false => Visibility::Hidden,
          true => Visibility::Inherited,
        };
      }
    }
  }
}
