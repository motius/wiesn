<template>
  <div id="app">
    <h1>{{greeting}}</h1>
  </div>
</template>

<script>
import { greet } from './lib.rs'

const greeting = greet()

export default {
  data: () => ({
    message: 'Hello, world'
  }),
  computed: {
    greeting: () => greet()
  }
}
</script>
