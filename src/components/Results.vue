<template>
  <div v-loading="resultLoading">
    <div id="chart" style="height: 500px; width:500px"></div>
    <el-card>
      <div v-if="benchmarkResults['benchmark_env']">
        <div>Benchmark Environments:</div>
        <div v-for="item in Object.entries(benchmarkResults['benchmark_env'])" :key="item[0]">
          <span class="result-category">{{item[0]}}</span>
          -{{item[1]}}
        </div>
      </div>
      <div v-if="benchmarkResults['basics']">
        <div>Basic Results:</div>
        <div v-for="item in Object.entries(benchmarkResults['basics'])" :key="item[0]">
          <span class="result-category">{{item[0]}}</span>
          -{{item[1]}}
        </div>
      </div>
      <div style="margin-top:1em;" v-if="benchmarkResults['pcm_results']">
        <div>PCM Results:</div>
        <div v-for="item in Object.entries(benchmarkResults['pcm_results'])" :key="item[0]">
          <span class="result-category">{{item[0]}}</span>
          -{{item[1]}}
        </div>
      </div>
    </el-card>
  </div>
</template>

<script>
import echarts from "echarts";
export default {
  name: "Results",
  props: {},
  data() {
    return { resultLoading: true, benchmarkResults: {} };
  },
  methods: {
    updateResults(results) {
      this.benchmarkResults = results;
      this.plotFigure(results["basics"]);
    },
    plotFigure(data) {
      let myChart = echarts.init(document.getElementById("chart"));
      myChart.setOption({
        title: {
          text: "PiBench Result"
        },
        tooltip: {},
        xAxis: {
          type: "category",
          data: data["samplings"].map((_, index) => {
            return data["sample_time"] * index;
          })
        },
        yAxis: {
          type: "value"
        },
        series: [
          {
            type: "line",
            data: data["samplings"]
          }
        ]
      });
      this.resultLoading = false;
    }
  }
};
</script>

<style scoped>
.result-category {
  font-weight: 700;
}
</style>

