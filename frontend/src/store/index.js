// Copyright (c) Simon Fraser University. All rights reserved.
// Licensed under the MIT license.

// Authors:
// Xiangpeng Hao <xiangpeng_hao@sfu.ca>

import Vue from 'vue'
import Vuex from 'vuex'
import createPersistedState from "vuex-persistedstate";

Vue.use(Vuex)

const state = {
  backends: [],
  showAbout: true,
  configPresets: [],
  benchmarkResults: []
};

const mutations = {
  addBackend(state, backend) {
    state.backends.push(backend);
  },
  setShowAbout(state, show) {
    state.showAbout = show;
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
