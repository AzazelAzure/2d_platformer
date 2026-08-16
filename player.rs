#[derive(Component)]
#[require(Sprite, Transform)]
struct Player;

impl Player {
    fn new() -> (Player, Sprite, Transform, Collider, 
}
// Player Constants
const PLAYER_COLOR: Color = Color::srgb(0.0, 1.0, 0.0);
const PLAYER_WALK_SPEED: f32 = 400.0;
const PLAYER_RUN_SPEED: f32 = 600.0;

const GRAVITY: f32 = -200.0;



