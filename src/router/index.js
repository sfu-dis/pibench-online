import Vue from 'vue'
import Router from 'vue-router'
Vue.use(Router)

import Benchmark from '@/components/Benchmark';
import Settings from '@/components/Settings';
import Analyze from '@/components/Analyze';

export default new Router({
    routes: [
        {
            path: '/',
            redirect: '/benchmark'
        },
        {
            path: '/analyze',
            name: 'Analyze',
            component: Analyze
        },
        {
            path: '/benchmark',
            name: 'Benchmark',
            component: Benchmark
        }, {
            path: "/settings",
            name: "Settings",
            component: Settings
        }
    ]
})
