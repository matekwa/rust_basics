//Enum are types with a few definite values

enum Movement{
    //variants
    Up,
    Down,
    Right,
    Left,
}
fn move_avatar(m: Movement){
    match m{
        Movement::Up => println!("Avatar Noving up"),
        Movement::Down => println!("Avatar Noving down"),
        Movement::Left => println!("Avatar Noving left"),
        Movement::Right => println!("Avatar Noving right"),
    }
}
pub fn run() {
    let avatar = Movement::Up;
    move_avatar(avatar);
}