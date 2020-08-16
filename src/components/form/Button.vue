<template>
  <button
    class="bg-blue-500 my-3 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
    @click="$emit('click', $event)"
    :disabled="disabled"
  >
    <slot></slot>
  </button>
</template>

<script>
export default {
  props: ['disabled'],
}
</script>
