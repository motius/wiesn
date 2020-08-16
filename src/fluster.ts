export default {
  install(Vue, options) {
    Vue.mixin({
      data: () => ({
        $state: options.state,
      }),
      methods: {
        $update: options.update,
      },
    })
  }
}
