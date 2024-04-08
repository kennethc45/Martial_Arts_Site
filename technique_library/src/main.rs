use technique_library::technique_catalog::*;

fn main() {
    let mut techniques: TechniqueLibrary = TechniqueLibrary::new();
    let roundhouse = Technique::new(
        "Roundhouse Kick".to_string(),
        TechniqueCategory::Strike(StrikeCategory::Kick),
        "A powerful and dynamic kick that involves rotating the hips and torso to deliver a devestating strike.".to_string()
    );

    let ghetto_kick = Technique::new(
        "ghetto kick".to_string(),
        TechniqueCategory::Strike(StrikeCategory::Kick),
        "A ghetto ### kick.".to_string()
    );

    techniques.add_technique(roundhouse.name.clone(), roundhouse.clone());

    match techniques.get_technique(&ghetto_kick.name) {
        Ok(technique) => {
            technique.display()
        }
        Err(err) => println!("{}",err)
    }

    match techniques.get_technique(&roundhouse.name) {
        Ok(technique) => {
            technique.display()
        }
        Err(err) => println!("{}",err)
    }
}
