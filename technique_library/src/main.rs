//use technique_library::*;
use technique_library::technique_catalog::*;

fn main() {
    let roundhouse = Technique::new(
        "Roundhouse Kick".to_string(),
        TechniqueCategory::Strike(StrikeCategory::Kick),
        "A powerful and dynamic kick that invoolves rotating the hips and torso to deliver a devestating strike.".to_string()
    );

    roundhouse.display();
}
