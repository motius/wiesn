<template>
  <div id="app">
    <Recipe></Recipe>
    <Equipment></Equipment>
    <Vitals></Vitals>
  </div>
</template>

<script>
import Recipe from './Recipe'
import Equipment from './Equipment'
import Vitals from './Vitals'

export default {
  components: {
    Recipe,
    Equipment,
    Vitals,
  },
}
</script>
