import Vue from 'vue'
import Router from 'vue-router'
Vue.use(Router)

import Benchmark from '@/App';

export default new Router({
    routes: [
        {
            path: '/',
            redirect: '/benchmark'
        },
        {
            path: '/benchmark',
            name: 'Benchmark',
            component: Benchmark
        },
    ]
})
