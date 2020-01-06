import Vue from 'vue'
import Vuex from 'vuex'
import createPersistedState from "vuex-persistedstate";

Vue.use(Vuex)

const state = {
  backends: [],
  configPresets: [],
  benchmarkResults: []
};

const mutations = {
  addBackend(state, backend) {
    state.backends.push(backend);
  },
  updateBackend(state, { index, backend }) {
    Vue.set(state.backends, index, backend);
  },

  deleteBackend(state, index) {
    state.backends.splice(index, 1);
  },
  addConfigPreset(state, preset) {
    state.configPresets.push(preset);
  },
  addBenchmarkResult(state, { result, param }) {
    state.benchmarkResults.push({ param, result });
  },
  updateBenchmarkResults(state, newResults) {
    state.benchmarkResults = newResults;
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
