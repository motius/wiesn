<template>
  <select
    class="block appearance-none w-full bg-white border border-gray-400 hover:border-gray-500 px-4 py-2 pr-8 rounded shadow leading-tight focus:outline-none focus:shadow-outline"
    :value="value"
    @input="$emit('input', $event.target.value)"
    @change="$emit('change', $event)"
  >
    <slot></slot>
  </select>
</template>

<script>
export default {
  props: ['value']
}
</script>
