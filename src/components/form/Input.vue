<template>
  <div class="md:flex md:items-center mb-6">
    <div class="md:w-1/3">
      <label
        class="block text-gray-500 font-bold md:text-right mb-1 md:mb-0 pr-4"
        :for="id"
      >
        {{label}}
      </label>
    </div>
    <div class="relative md:w-2/3">
      <input
        :id="id"
        class="bg-gray-200 appearance-none border-2 border-gray-200 rounded w-full py-2 px-4 text-gray-700 leading-tight focus:outline-none focus:bg-white focus:border-purple-500"
        :value="value"
        @input="$emit('input', $event.target.value)"
        @change="$emit('change', $event)"
        :type="type"
        :step="step"
      ></input>
      <span
        class="adornment"
        v-if="unit"
      >
        {{ unit }}
      </span>
    </div>
  </div>
</template>

<script>
export default {
  props: ['type', 'value', 'label', 'id', 'step', 'unit']
}
</script>

<style scoped>
.adornment {
  position: absolute;
  right: 38px;
  height: 100%;
  display: flex;
  justify-content: center;
  padding: 0.5rem 0;
  top: 0;
  opacity: 0.5;
}
</style>
