# PiBench Online

Online demo: http://pibench.org

![](src/assets/readme-demo.gif)

## What is PiBench Online?

PiBench Online is an interactive system for benchmarking PM indexes in a fair and reproducible manner, it is based on PiBench, the first unified, highly customizable benchmarking framework for PM indexes.

PiBench Online enables push-button evaluation of PM indexes: the user can upload a shared library that implements the index, set parameters to run customized benchmarks, and analyze results interactively, all through an easy-to-use web interface.

We have made PiBench Online available at http://pibench.org and hope it can promote fair comparison and reproducibility in future PM index research.

## About Us

PiBench Online is built by: [Xiangpeng Hao](https://haoxp.xyz), [Tianzheng Wang](https://www2.cs.sfu.ca/~tzwang/) ([Systems Group](http://systems.cs.sfu.ca/), Simon Fraser University), 
Lucas Lersch (TU Dresden & SAP SE) and Ismail Oukid (Snowflake Computing)

Read our paper: [Evaluating Persistent Memory Range Indexes](http://www.vldb.org/pvldb/vol13/p574-lersch.pdf)

Request PiBench resources: support@pibench.org

## Contribution

The PiBench Online frontend requires nodejs and the yarn package manager.

You also need to be familiar with [Vue.js](https://vuejs.org/) and [ElementUI](https://element.eleme.io/#/en-US) as well as [Echarts.js](echartsjs.com/).

#### Project setup
```
yarn install
```

#### Compiles and hot-reloads for development
```
yarn serve
```

#### Compiles and minifies for production
```
yarn build
```
