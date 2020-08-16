<template>
  <div>
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
  get_original_gravity,
  get_final_gravity,
  get_color,
} from './lib.rs'

export default {
  computed: {
    originalGravity: function() {
      return get_original_gravity(this.$data.$state.recipe, this.$data.$state.equipment).toFixed(4)
    },
    finalGravity: function() {
      return get_final_gravity(this.$data.$state.recipe, this.$data.$state.equipment).toFixed(4)
    },
    abv: function() {
      return (132.911 * ((this.originalGravity - this.finalGravity) / this.finalGravity)).toFixed(2)
    },
    color: function() {
      return get_color(this.$data.$state.recipe).toFixed(2)
    },
  },
}
</script>
