<template>
  <div id="app">
    <h1>PiBench Online</h1>

    <el-form ref="form-basic" :inline="true" :model="formBasic" label-width="10em">
      <el-form-item label="PiBench Backend">
        <el-select v-model="formBasic.backend" placeholder="Select Backend">
          <el-option
            v-for="item in backends"
            :key="item.value"
            :label="item.value"
            :value="item.value"
          ></el-option>
        </el-select>
      </el-form-item>
      <el-form-item label="Wrapper">
        <el-select v-model="formBasic.wrapper" placeholder="Select Wrapper">
          <el-option
            v-for="item in wrappers"
            :key="item.value"
            :label="item.value"
            :value="item.value"
          ></el-option>
        </el-select>
      </el-form-item>
    </el-form>
    <el-form ref="form-config" :model="piBenchParams" label-width="10em">
      <el-form-item label="Thread Count">
        <el-input-number v-model="piBenchParams.threadCount" :step="1" :min="1" :max="10"></el-input-number>
      </el-form-item>
      <el-form-item label="Operation Count">
        <el-input-number v-model="piBenchParams.opCount" :step="1" :min="1" :max="10"></el-input-number>
      </el-form-item>
      <el-form-item label="Load Count">
        <el-input-number v-model="piBenchParams.loadCount" :step="1" :min="1" :max="10"></el-input-number>
      </el-form-item>
      <el-form-item label="Read Ratio">
        <el-input-number v-model="piBenchParams.read" :step="0.1" :min="0" :max="1"></el-input-number>
      </el-form-item>
      <el-form-item label="Insert Ratio">
        <el-input-number v-model="piBenchParams.insert" :step="0.1" :min="0" :max="1"></el-input-number>
      </el-form-item>
      <el-form-item label="Update Ratio">
        <el-input-number v-model="piBenchParams.update" :step="0.1" :min="0" :max="1"></el-input-number>
      </el-form-item>
      <el-form-item label="Delete Ratio">
        <el-input-number v-model="piBenchParams.delete" :step="0.1" :min="0" :max="1"></el-input-number>
      </el-form-item>
      <el-form-item label="Env">
        <el-input type="textarea" v-model="piBenchParams.env"></el-input>
      </el-form-item>
      <el-form-item>
        <el-button type="primary">Start</el-button>
      </el-form-item>
    </el-form>

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
      backends: [{ value: "webassembly" }, { value: "localhost" }],
      formBasic: {},
      piBenchParams: {}
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
