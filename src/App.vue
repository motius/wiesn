<template>
  <div id="app">
    <input v-model="volume" type="number" />
    <h1>{{originalGravity}}</h1>
    <h2>Malts</h2>
    <div v-for="(maltInRecipe, index) in maltsInRecipe">
      <h3>{{maltInRecipe.malt.name}}</h3>
      <input @change="updateMaltMass(index, $event)" :value="maltInRecipe.mass" type="number" />
    </div>
    <select v-model="selectedMalt">
      <option value=''>Select Malt</option>
      <option v-for="availableMalt in availableMalts" :value="availableMalt.id">
        {{availableMalt.name}}
      </option>
      <button @click="addSelectedMalt" :disabled="selectedMalt === ''">Add Malt</button>
    </select>
  </div>
</template>

<script>
import { Recipe } from './lib.rs'

export default {
  data: () => ({
    recipe: Recipe.new(),
    selectedMalt: '',
  }),
  methods: {
    updateMaltMass: function(index, event) {
      this.recipe = this.recipe.update_malt_mass(index, event.currentTarget.value)
    },
    addSelectedMalt: function() {
      this.recipe = this.recipe.add_malt(this.selectedMalt)
      this.selectedMalt = ''
    }
  },
  computed: {
    volume: {
      get: function() {
        return this.recipe.get_volume()
      },
      set: function(newValue) {
        this.recipe = this.recipe.set_volume(newValue)
      },
    },
    originalGravity: function() {
      return this.recipe.get_original_gravity()
    },
    maltsInRecipe: function() {
      return JSON.parse(this.recipe.get_malts_in_recipe())
    },
    availableMalts: function() {
      return JSON.parse(this.recipe.get_available_malts())
    }
  },
}
</script>
