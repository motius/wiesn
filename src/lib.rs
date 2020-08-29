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

#[derive(Copy, Clone, Serialize)]
struct Hop {
    alpha_acid: f32,
    mass: f32,
    boil_time: f32,
}

#[wasm_bindgen]
#[derive(Serialize)]
pub struct Recipe {
    malts_in_recipe: Vec<MaltInRecipe>,
    volume: f32,
    yeast: Option<Yeast>,
    hops: Vec<Hop>,
}

static MALTS: [Malt; 2] = [
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

static YEASTS: [Yeast; 2] = [
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
        let hops = vec![];

        Recipe {
            malts_in_recipe,
            volume,
            yeast,
            hops,
        }
    }

    pub fn get_json(&self) -> String {
        json!(&self).to_string()
    }

    pub fn set_volume(&self, value: f32) -> Recipe {
        let malts_in_recipe = self.malts_in_recipe.clone();
        let volume = value;
        let yeast = self.yeast;
        let hops = self.hops.clone();

        Recipe {
            malts_in_recipe,
            volume,
            yeast,
            hops,
        }
    }

    pub fn update_malt_mass(&self, index: usize, new_mass: f32) -> Recipe {
        let mut malts_in_recipe = self.malts_in_recipe.clone();
        let volume = self.volume;
        let yeast = self.yeast;
        let hops = self.hops.clone();

        malts_in_recipe[index].mass = new_mass;

        Recipe {
            malts_in_recipe,
            volume,
            yeast,
            hops,
        }
    }

    pub fn add_malt(&self, malt_id: String) -> Recipe {
        let mut malts_in_recipe = self.malts_in_recipe.clone();
        let volume = self.volume;
        let yeast = self.yeast;
        let hops = self.hops.clone();

        return match MALTS.iter().find(|malt| malt.id == malt_id) {
            None => return Recipe {
                malts_in_recipe,
                volume,
                yeast,
                hops,
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
                    hops,
                };
            }
        }
    }

    pub fn add_hop(&self) -> Recipe {
        let malts_in_recipe = self.malts_in_recipe.clone();
        let volume = self.volume;
        let yeast = self.yeast;
        let mut hops = self.hops.clone();

        hops.push(Hop {
            mass: 0.0,
            alpha_acid: 0.0,
            boil_time: 0.0,
        });

        return Recipe {
            malts_in_recipe,
            volume,
            yeast,
            hops,
        }
    }

    pub fn update_hop_mass(&self, index: usize, new_value: f32) -> Recipe {
        let malts_in_recipe = self.malts_in_recipe.clone();
        let volume = self.volume;
        let yeast = self.yeast;
        let mut hops = self.hops.clone();

        hops[index].mass = new_value;

        return Recipe {
            malts_in_recipe,
            volume,
            yeast,
            hops,
        }
    }

    pub fn update_hop_boil(&self, index: usize, new_value: f32) -> Recipe {
        let malts_in_recipe = self.malts_in_recipe.clone();
        let volume = self.volume;
        let yeast = self.yeast;
        let mut hops = self.hops.clone();

        hops[index].boil_time = new_value;

        return Recipe {
            malts_in_recipe,
            volume,
            yeast,
            hops,
        }
    }

    pub fn update_hop_alpha_acid(&self, index: usize, new_value: f32) -> Recipe {
        let malts_in_recipe = self.malts_in_recipe.clone();
        let volume = self.volume;
        let yeast = self.yeast;
        let mut hops = self.hops.clone();

        hops[index].alpha_acid = new_value;

        return Recipe {
            malts_in_recipe,
            volume,
            yeast,
            hops,
        }
    }

    pub fn change_yeast(&self, yeast_id: String) -> Recipe {
        let malts_in_recipe = self.malts_in_recipe.clone();
        let volume = self.volume;
        let hops = self.hops.clone();

        return match YEASTS.iter().find(|yeast| yeast.id == yeast_id) {
            None => {
                let yeast = self.yeast;
                return Recipe {
                    malts_in_recipe,
                    volume,
                    yeast,
                    hops,
                };
            },
            Some(found_yeast) => {
                let yeast = Some(found_yeast.clone());
                return Recipe {
                    malts_in_recipe,
                    volume,
                    yeast,
                    hops,
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
    json!(MALTS).to_string()
}

#[wasm_bindgen]
pub fn get_available_yeasts() -> String {
    json!(YEASTS).to_string()
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

fn get_hop_bitterness(hop: &Hop, gravity: f32, volume: f32) -> f32 {
    let aau = hop.mass * hop.alpha_acid;
    let utilization = 1.65 * 0.000125_f32.powf(gravity - 1.0) * (1.0 - 2.71828_f32.powf(-0.04 * hop.boil_time)) / 4.15;

    aau * utilization * 11.0 / volume // 10 for milligrams + percent -> grams + fraction. 1.1 assumes pellets
}

#[wasm_bindgen]
pub fn get_bitterness(recipe: &Recipe, equipment: &Equipment) -> f32 {
    let gravity = get_original_gravity(recipe, equipment);
    recipe.hops.iter().fold(0.0, |sum, hop| sum + get_hop_bitterness(hop, gravity, recipe.volume))
}
