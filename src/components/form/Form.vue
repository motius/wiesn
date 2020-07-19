<template>
  <form class="w-full max-w-sm" @submit="handleSubmit">
    <slot></slot>
  </form>
</template>

<script>
export default {
  methods: {
    handleSubmit: function(event) {
      event.preventDefault()
    },
  }
}
</script>
