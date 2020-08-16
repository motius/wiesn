<template>
  <div id="app">
    <h2>Recipe</h2>
    <Form>
      <Input
        :id="volume"
        v-model="volume"
        type="number"
        label="Volume"
        step="1"
      /></Input>
      <h3>Malts</h3>
      <div v-for="(maltInRecipe, index) in maltsInRecipe">
        <h4>{{maltInRecipe.malt.name}}</h4>
        <Input
          :id="maltInRecipe.malt.id"
          @change="updateMaltMass(index, $event)"
          :value="maltInRecipe.mass"
          type="number"
          label="Mass"
        ></Input>
      </div>
      <Select v-model="selectedMalt">
        <option value=''>Select Malt</option>
        <option v-for="availableMalt in availableMalts" :value="availableMalt.id">
          {{availableMalt.name}}
        </option>
      </Select>
      <Button @click="addSelectedMalt" :disabled="selectedMalt === ''">Add Malt</Button>
      <h3>Yeast</h3>
      <Select v-model="selectedYeast">
        <option value=''>Select Yeast</option>
        <option v-for="availableYeast in availableYeasts" :value="availableYeast.id">
          {{availableYeast.name}}
        </option>
      </Select>
    </Form>
    <h2>Equipment</h2>
    <Form>
      <Input
        :id="efficiency"
        v-model="efficiency"
        type="number"
        label="Efficiency"
        step="1"
      /></Input>
    </Form>
    <h2>Vitals</h2>
    <ul>
      <li>OG: {{originalGravity}}</li>
      <li>FG: {{finalGravity}}</li>
      <li>ABV: {{abv}}%</li>
      <li>SRM: {{color}}</li>
      <div :style="{ position: 'relative', width: '500px' }">
        <div :style="{ position: 'absolute', left: 12.5 * color }">|</div>
        <div :style="{ background: 'linear-gradient(to right, #fee799, #db7d00, #963500, #5b0d01, #35090a)', width: '100%', height: '10px' }" ></div>
      </div>
    </ul>
  </div>
</template>

<script>
import {
  Recipe,
  Equipment,
  get_original_gravity,
  get_final_gravity,
  get_color,
} from './lib.rs'
import Input from './components/form/Input'
import Form from './components/form/Form'
import Select from './components/form/Select'
import Button from './components/form/Button'

export default {
  components: {
    Input,
    Form,
    Select,
    Button,
  },
  data: () => ({
    recipe: Recipe.new(),
    equipment: Equipment.new(),
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
    efficiency: {
      get: function() {
        return Math.round(this.equipment.get_efficiency() * 100)
      },
      set: function(newValue) {
        this.equipment = this.equipment.set_efficiency(newValue / 100)
      },
    },
    originalGravity: function() {
      return get_original_gravity(this.recipe, this.equipment).toFixed(4)
    },
    finalGravity: function() {
      return get_final_gravity(this.recipe, this.equipment).toFixed(4)
    },
    abv: function() {
      return (132.911 * ((this.originalGravity - this.finalGravity) / this.finalGravity)).toFixed(2)
    },
    color: function() {
      return get_color(this.recipe).toFixed(2)
    },
    maltsInRecipe: function() {
      return JSON.parse(this.recipe.get_malts_in_recipe())
    },
    availableMalts: function() {
      return JSON.parse(this.recipe.get_available_malts())
    },
    availableYeasts: function() {
      return JSON.parse(this.recipe.get_available_yeasts())
    },
    selectedYeast: {
      get: function() {
        const yeast = JSON.parse(this.recipe.get_yeast())
        if (yeast) {
          return yeast.id
        }
        return ''
      },
      set: function(newValue) {
        this.recipe = this.recipe.change_yeast(newValue)
      },
    },
  },
}
</script>
