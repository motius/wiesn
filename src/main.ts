import Vue from 'vue'
import App from './App.vue'
import { Recipe, Equipment } from './lib.rs'
import fluster from './fluster'

const state = {
  recipe: Recipe.new(),
  equipment: Equipment.new(),
}

const update = (name, value) => {
  state[name] = value
}

Vue.use(fluster, {
  state,
  update,
})

new Vue({
  render: h => h(App),
}).$mount('#app')
