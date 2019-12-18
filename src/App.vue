<template>
  <div id="app">
    <h1>PiBench Online</h1>
    <div class="basic-config">
      <el-select v-model="backendSelected" placeholder="Select Backend">
        <el-option
          v-for="item in backends"
          :key="item.value"
          :label="item.value"
          :value="item.value"
        ></el-option>
      </el-select>
      <el-select v-model="wrapperSelected" placeholder="Select Wrapper">
        <el-option
          v-for="item in wrappers"
          :key="item.value"
          :label="item.value"
          :value="item.value"
        ></el-option>
      </el-select>
    </div>

    <el-button type="primary">Primary</el-button>
    <div id="chart" style="height: 500px; width:500px"></div>
  </div>
</template>

<script>
import echarts from "echarts";

export default {
  name: "app",
  components: {},
  data() {
    return {
      wrappers: [
        { value: "std::set" },
        { value: "bztree" },
        { value: "fptree" }
      ],
      wrapperSelected: "",
      backends: [{ value: "webassembly" }, { value: "localhost" }],
      backendSelected: ""
    };
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
