<template>
  <div id="app">
    <HelloWorld msg="Welcome to Your Vue.js App" />
    <el-button type="primary">Primary</el-button>
    <div id="chart" style="height: 500px; width:500px"></div>
  </div>
</template>

<script>
import HelloWorld from "./components/HelloWorld.vue";
import echarts from 'echarts'

export default {
  name: "app",
  components: {
    HelloWorld
  },
  mounted() {

    // initialize echarts instance with prepared DOM
    let myChart = echarts.init(document.getElementById("chart"));
    // draw chart
    myChart.setOption({
      title: {
        text: "ECharts entry example"
      },
      tooltip: {},
      xAxis: {
        data: ["shirt", "cardign", "chiffon shirt", "pants", "heels", "socks"]
      },
      yAxis: {},
      series: [
        {
          name: "sales",
          type: "bar",
          data: [5, 20, 36, 10, 10, 20]
        }
      ]
    });
  }
};
</script>

<style>
#app {
  font-family: "Avenir", Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  color: #2c3e50;
}
</style>
