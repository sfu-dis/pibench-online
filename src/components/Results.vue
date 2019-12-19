<template>
  <div>
    <div id="chart" v-loading="resultLoading" style="height: 500px; width:500px"></div>
  </div>
</template>

<script>
import echarts from "echarts";
export default {
  name: "Results",
  props: {},
  data() {
    return { resultLoading: true };
  },
  methods: {
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
</style>

