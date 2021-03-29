use::bevy::prelude::*;
use::bevy::utils::Duration;
use::bevy::ecs::schedule::ShouldRun;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(hello_world.system())
        .add_startup_system(setup.system())
        .add_system(increment_frame.system())
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(run_on_frame::<0>.system())
                .with_system(frame_zero.system())
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(run_on_frame::<1>.system())
                .with_system(frame_one.system())
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(run_on_frame::<2>.system())
                .with_system(frame_two.system())
        )
        .run();
}

struct SequencerTimer(Timer);
impl Default for SequencerTimer {
    fn default() -> Self {
        Self(Timer::new(Duration::from_millis(0), true))
    }
}

struct Player;
struct CurrentFrame(i32);
struct Sequence {
    durations: Vec<u64>,
    timer: SequencerTimer,
}

fn setup(mut commands: Commands) {
    commands.spawn()
        .insert(Player)
        .insert(CurrentFrame(-1))
        .insert(Sequence {
            durations: vec![1000, 5000, 10000],
            timer: Default::default(),
        });
}

fn increment_frame(
    time: Res<Time>, 
    mut query: Query<(&mut CurrentFrame, &mut Sequence)>
) {
    for (mut current_frame, mut sequence) in query.iter_mut() {
        if !sequence.timer.0.tick(time.delta()).finished() { return };
        current_frame.0 += 1;
        if current_frame.0 > sequence.durations.len() as i32 - 1 {
            current_frame.0 = 0;   
        }
        sequence.timer.0.reset();
        let duration = sequence.durations[current_frame.0 as usize];
        sequence.timer.0.set_duration(Duration::from_millis(duration));
    }
}

fn hello_world() {
    println!("hello world!");
}

fn run_on_frame<const RUNFRAME: i32>(query: Query<&CurrentFrame, With<Player>>, mut last_frame: Local<i32>) -> ShouldRun {
    let mut result = ShouldRun::No;
    if let Ok( current_frame ) = query.single() {
        if current_frame.0 == RUNFRAME && current_frame.0 != *last_frame{
            result = ShouldRun::Yes;
        }
        *last_frame = current_frame.0;
    }
    result
}

fn frame_zero() {
    println!("frame zero");
}

fn frame_one() {
    println!("frame one");
}

fn frame_two() {
    println!("frame two");
}