export default {
  install(Vue, options) {
    Vue.mixin({
      data: () => ({
        $state: options.state,
        $test123: '123',
      }),
      methods: {
        $update: options.update,
      },
    })
  }
}
