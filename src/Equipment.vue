<template>
  <Form heading="Equipment">
    <Input
      :id="efficiency"
      :value="efficiency"
      @change="setEfficiency"
      type="number"
      label="Efficiency"
      step="1"
      unit="%"
    /></Input>
  </Form>
</template>

<script>
import Input from './components/form/Input'
import Form from './components/form/Form'
import Select from './components/form/Select'
import Button from './components/form/Button'

export default {
  components: {
    Input,
    Form,
  },
  methods: {
    setEfficiency: function(e) {
      this.$update('equipment', this.$data.$state.equipment.set_efficiency(e.target.value / 100))
    },
  },
  computed: {
    obj: function() {
      return JSON.parse(this.$data.$state.equipment.get_json())
    },
    efficiency: function() {
      return Math.round(this.obj.efficiency * 100)
    },
  },
}
</script>
