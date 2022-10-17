import Vue from 'vue'
import Vuex from 'vuex'
import app from "./modules/app";
import mbcontroller from "@/store/modules/mbcontroller";

import apiMethods from "./modules/abisMethods"
Vue.use(Vuex)

export default new Vuex.Store({
  state: {
  },
  getters: {
  },
  mutations: {
  },
  actions: {
  },
  modules: {
    app,
    mbcontroller,
    ...apiMethods
  }
})
