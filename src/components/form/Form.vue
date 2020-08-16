<template>
  <DisplayBox :heading="heading">
    <form @submit="handleSubmit">
      <slot></slot>
    </form>
  </DisplayBox>
</template>

<script>
import DisplayBox from '../DisplayBox'

export default {
  components: {
    DisplayBox,
  },
  props: [
    'heading',
  ],
  methods: {
    handleSubmit: function(event) {
      event.preventDefault()
    },
  }
}
</script>
