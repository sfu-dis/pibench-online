import Vue from 'vue'
import Vuex from 'vuex'
import createPersistedState from "vuex-persistedstate";

Vue.use(Vuex)

const state = {
  backends: []
};

const mutations = {
  addBackend(state, backend) {
    state.backends.push(backend)
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
