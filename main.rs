use std::collections::HashMap;

type Collection = HashMap<String, Vec<String>>;

pub fn add_animal_to_section(animal: &str, section: &str, registry: &mut Collection) {
    let animals = registry.entry(section.to_string()).or_insert(Vec::new());

    if !animals.contains(&animal.to_string()) {
        animals.push(animal.to_string());
    }
}

pub fn get_animals_in_section(section: &str, registry: &Collection) -> Vec<String> {
    if let Some(animals) = registry.get(section) {
        let mut animals = animals.clone();
        animals.sort();
        return animals;
    } else {
        return Vec::new();
    }
}

pub fn get_all_animals_sorted(registry: &Collection) -> Vec<String> {
    let mut all_animals: Vec<String> = Vec::new();

    for animal in registry.values() {
        all_animals.extend_from_slice(animal.as_slice());
    }
    all_animals.sort();

    return all_animals;
}
