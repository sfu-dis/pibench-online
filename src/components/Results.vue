<!--
Copyright (c) Simon Fraser University. All rights reserved.
Licensed under the MIT license.

Authors:
Xiangpeng Hao <xiangpeng_hao@sfu.ca>
-->

<template>
  <div class="container">
    <div
      id="chart"
      style="height: 400px; width:500px"
    ></div>
    <div>
      <div v-if="benchmarkResults['benchmark_env']">
        <div class="sub-title">Benchmark Environments:</div>
        <div
          v-for="item in Object.entries(benchmarkResults['benchmark_env'])"
          :key="item[0]"
        >
          <div style="display: flex;">
            <div class="result-category">{{item[0]}}</div>
            <div class="result-value">{{item[1]}}</div>
          </div>
        </div>
      </div>
      <div v-if="benchmarkResults['basics']">
        <div class="sub-title">Basic Results:</div>
        <div
          v-for="item in basicResults"
          :key="item[0]"
        >
          <div style="display: flex;">
            <div class="result-category">{{item[0]}}</div>
            <div class="result-value">{{item[1]}}</div>
          </div>
        </div>
      </div>
      <div
        style="margin-top:1em;"
        v-if="benchmarkResults['pcm_results']"
      >
        <div>PCM Results:</div>
        <div
          v-for="item in Object.entries(benchmarkResults['pcm_results'])"
          :key="item[0]"
        >
          <span class="result-category">{{item[0]}}</span>
          {{item[1]}}
        </div>
      </div>
    </div>
    <section style="margin-top: 1em;">
      <el-button
        size="small"
        @click="saveResult"
      >Save Result</el-button>
    </section>
  </div>
</template>

<script>
import echarts from "echarts";
import { mapMutations } from "vuex";
let cloneDeep = require("lodash/cloneDeep");

export default {
  name: "Results",
  props: {},
  data() {
    return { benchmarkResults: {}, benchmarkParams: {} };
  },
  computed: {
    basicResults() {
      return Object.entries(this.benchmarkResults["basics"]).filter(
        item => item[0] !== "samplings" && item[0] !== "latency"
      );
    }
  },
  methods: {
    ...mapMutations(["addBenchmarkResult"]),
    updateResults(results, params) {
      this.benchmarkResults = results;
      this.benchmarkParams = params;
      console.log(results);
      this.plotFigure(results);
    },
    saveResult() {
      this.addBenchmarkResult({
        result: cloneDeep(this.benchmarkResults),
        param: cloneDeep(this.benchmarkParams)
      });
      this.$message("Benchmark result saved!");
    },
    plotFigure(data) {
      let myChart = echarts.init(document.getElementById("chart"));
      let latencyKeys = Object.keys(data.results.latency);
      let latencyVals = Object.values(data.results.latency);
      myChart.setOption({
        title: {
          text: "Benchmark Result"
        },
        tooltip: {
          trigger: "axis"
        },
        toolbox: {
          feature: {
            dataView: { show: true, readOnly: false, title: "Dataview" },
            restore: { show: true, title: "Restore" },
            saveAsImage: { show: true, title: "Save As Image" }
          }
        },
        grid: [
          {
            height: "32%"
          },
          {
            top: "60%",
            height: "32%"
          }
        ],
        xAxis: [
          {
            type: "category",
            name: "Time",
            data: data.results["samples"].map((_, index) => {
              return data.options.sampling * index;
            }),
            gridIndex: 0,
            axisLine: { onZero: true }
          },
          {
            type: "category",
            name: "Sampling",
            data: latencyKeys,
            gridIndex: 1,
            axisLine: { onZero: true }
            // boundaryGap: false
          }
        ],
        yAxis: [
          {
            type: "value",
            name: "Throughput",
            axisLabel: {
              formatter: value => {
                return (
                  (((value / 1000000) * 1000) / data.options.sampling).toFixed(
                    2
                  ) + " M"
                );
              }
            },
            gridIndex: 0
          },
          {
            type: "value",
            name: "Latency (ns)",
            inverse: true,
            gridIndex: 1,
            axisLabel: {
              formatter: value => {
                return value / 1000 + " us";
              }
            }
          }
        ],
        series: [
          {
            type: "line",
            data: data.results.samples,
            xAxisIndex: 0,
            yAxisIndex: 0,
            name: "Samplings"
          },
          {
            type: "line",
            data: latencyVals,
            xAxisIndex: 1,
            yAxisIndex: 1,
            name: "Latency"
          }
        ]
      });
    }
  }
};
</script>

<style scoped>
.result-category {
  margin-right: 1em;
  min-width: 10em;
}

.result-value {
  max-width: 15em;
}

.sub-title {
  font-weight: 700;
  margin-top: 1em;
}

.container {
  font-family: consolas;
}
</style>

