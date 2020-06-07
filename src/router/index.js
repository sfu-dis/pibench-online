// Copyright (c) Simon Fraser University. All rights reserved.
// Licensed under the MIT license.

// Authors:
// Xiangpeng Hao <xiangpeng_hao@sfu.ca>

import Vue from 'vue'
import Router from 'vue-router'
Vue.use(Router)

import Benchmark from '@/components/Benchmark';
import Settings from '@/components/Settings';
import Analyze from '@/components/Analyze';
const Parser = () => import('@/components/Parser')


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
        }, {
            path: "/parser",
            name: "Parser",
            component: Parser
        },
    ]
})
