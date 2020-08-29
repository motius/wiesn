<template>
  <DisplayBox heading="Vitals">
    <ul>
      <li>OG: {{originalGravity}}</li>
      <li>FG: {{finalGravity}}</li>
      <li>ABV: {{abv}}%</li>
      <li>IBU: {{bitterness}}</li>
      <li>SRM: {{color}}</li>
      <div :style="{ position: 'relative', width: '500px' }">
        <div :style="{ position: 'absolute', left: 12.5 * color, top: -2, height: '14px', width: '3px', backgroundColor: 'black' }"></div>
        <div :style="{ background: 'linear-gradient(to right, #fee799, #db7d00, #963500, #5b0d01, #35090a)', width: '100%', height: '10px' }" ></div>
      </div>
    </ul>
  </DisplayBox>
</template>

<script>
import {
  get_original_gravity,
  get_final_gravity,
  get_color,
  get_bitterness,
} from './lib.rs'
import DisplayBox from './components/DisplayBox'

export default {
  components: {
    DisplayBox,
  },
  computed: {
    originalGravity: function() {
      return get_original_gravity(this.$data.$state.recipe, this.$data.$state.equipment).toFixed(3)
    },
    finalGravity: function() {
      return get_final_gravity(this.$data.$state.recipe, this.$data.$state.equipment).toFixed(3)
    },
    abv: function() {
      return (132.911 * ((this.originalGravity - this.finalGravity) / this.finalGravity)).toFixed(2)
    },
    color: function() {
      return get_color(this.$data.$state.recipe).toFixed(2)
    },
    bitterness: function() {
      return get_bitterness(this.$data.$state.recipe, this.$data.$state.equipment).toFixed(2)
    },
  },
}
</script>
