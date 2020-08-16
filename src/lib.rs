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
struct Yeast {
    id: &'static str,
    name: &'static str,
    attenuation: f32,
}

#[derive(Copy, Clone, Serialize)]
struct MaltInRecipe {
    malt: Malt,
    mass: f32,
}

#[wasm_bindgen]
#[derive(Serialize)]
pub struct Recipe {
    malts_in_recipe: Vec<MaltInRecipe>,
    volume: f32,
    yeast: Option<Yeast>,
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

static yeasts: [Yeast; 2] = [
    Yeast {
        id: "w-34/70",
        name: "Saflager W-34/70",
        attenuation: 0.82,
    },
    Yeast {
        id: "k-97",
        name: "Safale K-97",
        attenuation: 0.82,
    }
];

#[wasm_bindgen]
impl Recipe {
    pub fn new() -> Recipe {
        let volume = 23.0;
        let malts_in_recipe = vec![];
        let yeast = None;

        Recipe {
            malts_in_recipe,
            volume,
            yeast,
        }
    }


    pub fn get_json(&self) -> String {
        json!(&self).to_string()
    }

    pub fn set_volume(&self, value: f32) -> Recipe {
        let malts_in_recipe = self.malts_in_recipe.clone();
        let volume = value;
        let yeast = self.yeast;

        Recipe {
            malts_in_recipe,
            volume,
            yeast,
        }
    }

    pub fn update_malt_mass(&self, index: usize, new_mass: f32) -> Recipe {
        let mut malts_in_recipe = self.malts_in_recipe.clone();
        let volume = self.volume;
        let yeast = self.yeast;

        malts_in_recipe[index].mass = new_mass;

        Recipe {
            malts_in_recipe,
            volume,
            yeast,
        }
    }

    pub fn add_malt(&self, maltId: String) -> Recipe {
        let mut malts_in_recipe = self.malts_in_recipe.clone();
        let volume = self.volume;
        let yeast = self.yeast;

        return match malts.iter().find(|malt| malt.id == maltId) {
            None => return Recipe {
                malts_in_recipe,
                volume,
                yeast,
            },
            Some(found_malt) => {
                malts_in_recipe.push(MaltInRecipe {
                    malt: found_malt.clone(),
                    mass: 0.0,
                });

                return Recipe {
                    malts_in_recipe,
                    volume,
                    yeast,
                };
            }
        }
    }

    pub fn change_yeast(&self, yeast_id: String) -> Recipe {
        let mut malts_in_recipe = self.malts_in_recipe.clone();
        let volume = self.volume;

        return match yeasts.iter().find(|yeast| yeast.id == yeast_id) {
            None => {
                let yeast = self.yeast;
                return Recipe {
                    malts_in_recipe,
                    volume,
                    yeast,
                };
            },
            Some(found_yeast) => {
                let yeast = Some(found_yeast.clone());
                return Recipe {
                    malts_in_recipe,
                    volume,
                    yeast,
                };
            }
        }
    }
}

#[wasm_bindgen]
#[derive(Serialize)]
pub struct Equipment {
    efficiency: f32,
}

#[wasm_bindgen]
impl Equipment {
    pub fn new() -> Equipment {
        Equipment {
            efficiency: 0.7,
        }
    }

    pub fn get_json(&self) -> String {
        json!(&self).to_string()
    }

    pub fn set_efficiency(&self, value: f32) -> Equipment {
        let efficiency = value;

        Equipment {
            efficiency,
        }
    }
}

#[wasm_bindgen]
pub fn get_available_malts() -> String {
    json!(malts).to_string()
}

#[wasm_bindgen]
pub fn get_available_yeasts() -> String {
    json!(yeasts).to_string()
}

#[wasm_bindgen]
pub fn get_original_gravity(recipe: &Recipe, equipment: &Equipment) -> f32 {
    let total_yield = recipe.malts_in_recipe.iter().fold(0.0, |sum, malt| sum + malt.malt.max_yield * equipment.efficiency * malt.mass);
    1.0 + (total_yield / recipe.volume) / 1000.0
}

#[wasm_bindgen]
pub fn get_final_gravity(recipe: &Recipe, equipment: &Equipment) -> f32 {
    let og = get_original_gravity(recipe, equipment);
    return match recipe.yeast {
        None => og,
        Some(yeast) => {
            let attenuation = yeast.attenuation;

            og - attenuation * (og - 1.0)
        },
    };
}

#[wasm_bindgen]
pub fn get_color(recipe: &Recipe) -> f32 { // srm
    let mcu = recipe.malts_in_recipe.iter().fold(0.0, |sum, malt| sum + malt.malt.color * malt.mass) / recipe.volume;
    6.53224 * mcu.powf(0.6959)
}
