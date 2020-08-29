<template>
  <div class="relative m-6 p-6 w-full max-w-xl border rounded border-gray-300">
    <h2 class="bg-white mx-5 p-2 absolute left-0 top-0 transform -translate-y-1/2 border rounded-full border-gray-300">{{heading}}</h2>
    <slot></slot>
  </div>
</template>

<script>
export default {
  props: ['heading'],
}
</script>
