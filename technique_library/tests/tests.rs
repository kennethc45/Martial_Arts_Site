#[cfg(test)]
mod tests {
    use technique_library::technique_catalog::*;
    
    #[test]
    fn test_add_and_get_technique() {
        let mut techniques: TechniqueLibrary = TechniqueLibrary::new();
        let roundhouse = Technique::new(
            "Roundhouse Kick".to_string(),
            TechniqueCategory::Strike(StrikeCategory::Kick),
            "A powerful and dynamic kick that involves rotating the hips and torso to deliver a devestating strike.".to_string()
        );

        techniques.add_technique(roundhouse.name.clone(), roundhouse.clone());

        match techniques.get_technique(&roundhouse.name) {
            Ok(technique) => {
                assert_eq!(*technique, roundhouse, "Failed to retrieve roundhouse from the hashmap");
            }
            Err(err) => println!("{}",err)
        }
    }
    
    #[test]
    fn test_failed_get_technique() {
        let techniques: TechniqueLibrary = TechniqueLibrary::new();
        let ghetto_kick = Technique::new(
            "ghetto kick".to_string(),
            TechniqueCategory::Strike(StrikeCategory::Kick),
            "A ghetto ### kick.".to_string()
        );

        match techniques.get_technique(&ghetto_kick.name) {
            Ok(_) => {
                panic!("Expected error, technique was never added to the hashmap.")
            }
            Err(err) => println!("{}",err)
        }
    }

    #[test]
    fn test_update_technique() {
        let mut techniques: TechniqueLibrary = TechniqueLibrary::new();
        let ghetto_kick = Technique::new(
            "ghetto kick".to_string(),
            TechniqueCategory::Strike(StrikeCategory::Kick),
            "A ghetto ### kick.".to_string()
        );

        let ghetto_kick2 = Technique::new(
            "ghetto kick".to_string(),
            TechniqueCategory::Strike(StrikeCategory::Kick),
            "Street fight kick.".to_string()
        );

        techniques.add_technique(ghetto_kick.name.clone(), ghetto_kick.clone());
        techniques.update_technique(&ghetto_kick.name, ghetto_kick2.clone());

        match techniques.get_technique(&ghetto_kick2.name) {
            Ok(technique) => {
                assert_eq!(*technique, ghetto_kick2, "Hashmap entry for ghetto_kick should be the same as ghetto_kick2")
            }
            Err(err) => println!("{}",err)
        }
    }

    #[test]
    fn test_delete_technique() {
        let mut techniques: TechniqueLibrary = TechniqueLibrary::new();
        let ghetto_kick = Technique::new(
            "ghetto kick".to_string(),
            TechniqueCategory::Strike(StrikeCategory::Kick),
            "A ghetto ### kick.".to_string()
        );

        techniques.add_technique(ghetto_kick.name.clone(), ghetto_kick.clone());
        techniques.delete_technique(&ghetto_kick.name);

        match techniques.get_technique(&ghetto_kick.name) {
            Ok(_) => {
                panic!("Expected error, ghetto_kick should've been removed from the hashmap.")
            }
            Err(err) => println!("{}",err)
        }
    }
}
