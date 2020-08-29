<template>
  <div id="app" class="flex flex-row justify-around">
    <div>
      <Recipe></Recipe>
      <Equipment></Equipment>
    </div>
    <div>
      <Vitals></Vitals>
    </div>
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
