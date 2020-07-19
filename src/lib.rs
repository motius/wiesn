use wasm_bindgen::prelude::*;
use serde_derive::Serialize;
use serde_json::json;

#[derive(Copy, Clone, Serialize)]
struct Malt {
    id: &'static str,
    name: &'static str,
    max_yield: f32, // pt / kg / L
    color: f32, // Lovibond
}

#[derive(Copy, Clone, Serialize)]
struct MaltInRecipe {
    malt: Malt,
    mass: f32,
}

#[wasm_bindgen]
struct Recipe {
    malts_in_recipe: Vec<MaltInRecipe>,
    volume: f32,
}

static malts: [Malt; 2] = [
    Malt {
        id: "2-row-barley",
        name: "2 Row Barley",
        max_yield: 370.0,
        color: 2.0,
    },
    Malt {
        id: "munich",
        name: "Munich Malt",
        max_yield: 350.0,
        color: 10.0,
    }
];

#[wasm_bindgen]
impl Recipe {
    pub fn new() -> Recipe {
        let volume = 23.0;
        let malts_in_recipe = vec![];

        Recipe {
            malts_in_recipe,
            volume,
        }
    }

    pub fn get_original_gravity(&self) -> f32 {
        let total_yield = self.malts_in_recipe.iter().fold(0.0, |sum, malt| sum + malt.malt.max_yield * malt.mass);
        1.0 + (total_yield / self.volume) / 1000.0
    }

    pub fn get_volume(&self) -> f32 {
        self.volume
    }

    pub fn set_volume(&self, value: f32) -> Recipe {
        let malts_in_recipe = self.malts_in_recipe.clone();
        let volume = value;

        Recipe {
            malts_in_recipe,
            volume,
        }
    }

    pub fn get_malts_in_recipe(&self) -> String {
        json!(self.malts_in_recipe).to_string()
    }

    pub fn update_malt_mass(&self, index: usize, new_mass: f32) -> Recipe {
        let mut malts_in_recipe = self.malts_in_recipe.clone();
        let volume = self.volume;

        malts_in_recipe[index].mass = new_mass;

        Recipe {
            malts_in_recipe,
            volume,
        }
    }

    pub fn add_malt(&self, maltId: String) -> Recipe {
        let mut malts_in_recipe = self.malts_in_recipe.clone();
        let volume = self.volume;

        return match malts.iter().find(|malt| malt.id == maltId) {
            None => return Recipe {
                malts_in_recipe,
                volume,
            },
            Some(found_malt) => {
                malts_in_recipe.push(MaltInRecipe {
                    malt: found_malt.clone(),
                    mass: 0.0,
                });

                return Recipe {
                    malts_in_recipe,
                    volume,
                };
            }
        }
    }

    pub fn get_available_malts(&self) -> String {
        json!(malts).to_string()
    }
}
