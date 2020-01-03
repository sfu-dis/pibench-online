<template>
  <div class="container" v-loading="resultLoading">
    <div id="chart" style="height: 400px; width:500px"></div>
    <div>
      <div v-if="benchmarkResults['benchmark_env']">
        <div class="sub-title">Benchmark Environments:</div>
        <div v-for="item in Object.entries(benchmarkResults['benchmark_env'])" :key="item[0]">
          <div style="display: flex;">
            <div class="result-category">{{item[0]}}</div>
            <div class="result-value">{{item[1]}}</div>
          </div>
        </div>
      </div>
      <div v-if="benchmarkResults['basics']">
        <div class="sub-title">Basic Results:</div>
        <div v-for="item in basicResults" :key="item[0]">
          <div style="display: flex;">
            <div class="result-category">{{item[0]}}</div>
            <div class="result-value">{{item[1]}}</div>
          </div>
        </div>
      </div>
      <div style="margin-top:1em;" v-if="benchmarkResults['pcm_results']">
        <div>PCM Results:</div>
        <div v-for="item in Object.entries(benchmarkResults['pcm_results'])" :key="item[0]">
          <span class="result-category">{{item[0]}}</span>
          {{item[1]}}
        </div>
      </div>
    </div>
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
  computed: {
    basicResults() {
      return Object.entries(this.benchmarkResults["basics"]).filter(
        item => item[0] !== "samplings"
      );
    }
  },
  methods: {
    updateResults(results) {
      this.benchmarkResults = results;
      console.log(results);
      this.plotFigure(results["basics"]);
    },
    plotFigure(data) {
      let myChart = echarts.init(document.getElementById("chart"));
      myChart.setOption({
        title: {
          text: "Benchmark Result"
        },
        tooltip: {},
        xAxis: {
          type: "category",
          name: "Time",
          data: data["samplings"].map((_, index) => {
            return this.benchmarkResults["sample_time"] * index;
          })
        },
        yAxis: {
          type: "value",
          name: "Throughput",
          axisLabel: {
            formatter: value => {
              return (
                ((value / 1000000) * 1000) /
                  this.benchmarkResults["sample_time"].toFixed(2) +
                " M"
              );
            }
          }
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

