<template>
  <div>
    <el-card>
      <h1>All Benchmark Results</h1>
      <el-table :data="benchmarkResults" @selection-change="handleSelectionChange">
        <el-table-column fixed type="selection" width="55"></el-table-column>
        <el-table-column
          v-for="item in tableOptions"
          :fixed="item['fixed']"
          :prop="item['prop']"
          :label="item['label']"
          :key="item['label']"
        ></el-table-column>
      </el-table>
      <section style="margin-top:1em; margin-bottom: 2em;">
        <el-button @click="deleteSelected" size="mini" plain type="danger">Delete Selected</el-button>
      </section>
      <section>
        <el-select value-key="label" v-model="figureSelect.x" size="mini" placeholder="X axis">
          <el-option
            v-for="op in tableOptions"
            :label="op.label"
            :value="op"
            :key="`x-${op.label}`"
          ></el-option>
        </el-select>
        <el-select value-key="label" v-model="figureSelect.y" size="mini" placeholder="Y axis">
          <el-option
            v-for="op in tableOptions"
            :label="op.label"
            :value="op"
            :key="`y-${op.label}`"
          ></el-option>
        </el-select>
        <el-button size="mini" plain type="primary" @click="plotFigure">Plot figure</el-button>
      </section>
      <section>
        <div id="analyze-chart" style="height: 400px; width:500px"></div>
      </section>
    </el-card>
  </div>
</template>

<script>
import { mapState, mapMutations } from "vuex";
import echarts from "echarts";
var get = require("lodash/get");

export default {
  name: "Analyze",
  props: {},
  data() {
    return {
      currentSelected: {},
      tableOptions: [
        {
          prop: "param.basic.wrapper",
          label: "Wrapper"
        },
        {
          prop: "param.params.thread_cnt",
          label: "Thread"
        },
        { prop: "param.params.distribution", label: "Distribution" },
        { prop: "param.params.read", label: "Read" },
        { prop: "param.params.insert", label: "Insert" },
        { prop: "param.params.update", label: "Update" },
        { prop: "param.params.delete", label: "Delete" },
        { prop: "param.params.key_size", label: "Key Size" },
        { prop: "param.params.value_size", label: "Value Size" },
        { prop: "param.params.op_cnt", label: "OP Count" },
        { prop: "result.basics.Throughput", label: "Throughput" }
      ],
      figureSelect: {}
    };
  },
  computed: {
    ...mapState(["benchmarkResults"])
  },
  methods: {
    ...mapMutations(["addBenchmarkResult", "updateBenchmarkResults"]),
    handleSelectionChange(val) {
      this.currentSelected = val;
    },
    deleteSelected() {
      let self = this;
      const newBenchResults = this.benchmarkResults.filter(item => {
        return self.currentSelected.indexOf(item) === -1;
      });
      this.updateBenchmarkResults(newBenchResults);
    },
    plotFigure() {
      let self = this;
      let myChart = echarts.init(document.getElementById("analyze-chart"));

      const xValues = this.currentSelected.map(item => {
        return get(item, self.figureSelect.x.prop);
      });
      const yValues = this.currentSelected.map(item => {
        return get(item, self.figureSelect.y.prop);
      });
      console.log(xValues, yValues, myChart);
      myChart.setOption({
        title: {
          text: "Benchmark Result"
        },
        grid: { left: "15%", right: "15%" },
        tooltip: {},
        xAxis: {
          name: this.figureSelect.x.label,
          data: xValues
        },
        yAxis: {
          name: this.figureSelect.y.label
        },
        series: [
          {
            type: "line",
            data: yValues
          }
        ]
      });
    }
  }
};
</script>

<style scoped>
</style>
