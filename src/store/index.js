import Vue from 'vue'
import Vuex from 'vuex'
import createPersistedState from "vuex-persistedstate";

Vue.use(Vuex)

const state = {
  backends: [],
  configPresets: []
};

const mutations = {
  addBackend(state, backend) {
    state.backends.push(backend)
  },
  addConfigPreset(state, preset) {
    state.configPresets.push(preset);
  }
}

export default new Vuex.Store({
  state,
  mutations,
  actions: {
  },
  modules: {
  },
  plugins: [createPersistedState()]
})
