<template>
  <Form heading="Recipe">
    <Input
      :id="volume"
      :value="volume"
      @change="setVolume"
      type="number"
      label="Volume"
      step="1"
      unit="L"
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
        unit="kg"
      ></Input>
    </div>
    <Select v-model="selectedMalt">
      <option value=''>Select Malt</option>
      <option v-for="availableMalt in availableMalts" :value="availableMalt.id">
        {{availableMalt.name}}
      </option>
    </Select>
    <Button @click="addSelectedMalt" :disabled="selectedMalt === ''">Add Malt</Button>
    <h3>Hops</h3>
    <div v-for="(hop, index) in hops">
      <Input
        :id="'mass-' + index"
        @change="updateHopMass(index, $event)"
        :value="hop.mass"
        type="number"
        label="Mass"
        unit="g"
      ></Input>
      <Input
        :id="'acid-' + index"
        @change="updateHopAlphaAcid(index, $event)"
        :value="hop.alpha_acid.toFixed(1)"
        type="number"
        label="Alpha Acid"
        step="0.1"
        unit="%"
      ></Input>
      <Input
        :id="'boil-' + index"
        @change="updateHopBoil(index, $event)"
        :value="hop.boil_time"
        type="number"
        label="Boil Time"
        unit="min"
      ></Input>
    </div>
    <Button @click="addHop">Add Hop</Button>
    <h3>Yeast</h3>
    <Select :value="selectedYeast" @change="changeYeast">
      <option value=''>Select Yeast</option>
      <option v-for="availableYeast in availableYeasts" :value="availableYeast.id">
        {{availableYeast.name}}
      </option>
    </Select>
  </Form>
</template>

<script>
import Input from './components/form/Input'
import Form from './components/form/Form'
import Select from './components/form/Select'
import Button from './components/form/Button'
import { get_available_malts, get_available_yeasts } from './lib.rs'

export default {
  components: {
    Input,
    Form,
    Select,
    Button,
  },
  data: () => ({
    selectedMalt: '',
  }),
  methods: {
    updateMaltMass: function(index, event) {
      this.$update('recipe', this.$data.$state.recipe.update_malt_mass(index, event.currentTarget.value))
    },
    addHop: function() {
      this.$update('recipe', this.$data.$state.recipe.add_hop())
    },
    updateHopMass: function(index, event) {
      this.$update('recipe', this.$data.$state.recipe.update_hop_mass(index, event.currentTarget.value))
    },
    updateHopBoil: function(index, event) {
      this.$update('recipe', this.$data.$state.recipe.update_hop_boil(index, event.currentTarget.value))
    },
    updateHopAlphaAcid: function(index, event) {
      this.$update('recipe', this.$data.$state.recipe.update_hop_alpha_acid(index, event.currentTarget.value))
    },
    addSelectedMalt: function() {
      this.$update('recipe', this.$data.$state.recipe.add_malt(this.selectedMalt))
      this.selectedMalt = ''
    },
    setVolume: function(e) {
      this.$update('recipe', this.$data.$state.recipe.set_volume(e.target.value))
    },
    changeYeast: function(e) {
      this.$update('recipe', this.$data.$state.recipe.change_yeast(e.target.value))
    },
  },
  computed: {
    obj: function() {
      return JSON.parse(this.$data.$state.recipe.get_json())
    },
    volume: function() {
      return this.obj.volume
    },
    maltsInRecipe: function() {
      return this.obj.malts_in_recipe
    },
    hops: function() {
      return this.obj.hops
    },
    availableMalts: function() {
      return JSON.parse(get_available_malts())
    },
    availableYeasts: function() {
      return JSON.parse(get_available_yeasts())
    },
    selectedYeast: function() {
      if (this.obj.yeast) {
        return this.obj.yeast.id
      }
      return ''
    },
  },
}
</script>
