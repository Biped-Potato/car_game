use bevy::{diagnostic::*, prelude::*};
#[derive(Component)]
pub struct TimerText
{
    pub value : f32,
    
}
#[derive(Resource,Default)]
pub struct Completion
{
    pub started : bool,
    pub finished : bool,
}
pub fn text_update_system(
    completion : Res<Completion>,
    diagnostics: Res<Diagnostics>,
    time : Res<Time>,
    mut query: Query<(&mut Text, &mut TimerText)>,
) {
    for (mut text,mut timer_text) in query.iter_mut() {
        // Update the value of the second section
        if completion.started && !completion.finished
        {
            timer_text.value+=time.delta_seconds();
            text.sections[1].value = timer_text.value.to_string();
        }
    }
}
