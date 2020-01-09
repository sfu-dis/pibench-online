<template>
  <div>
    <el-card>
      <h1>All Benchmark Results</h1>
      <el-table size="small" :data="benchmarkResults" @selection-change="handleSelectionChange">
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
      <section style="margin-top: 3em;">
        <el-form inline>
          <el-form-item label="X axis: ">
            <el-select value-key="label" v-model="figureSelect.x" size="mini" placeholder="X axis">
              <el-option
                v-for="op in tableOptions"
                :label="op.label"
                :value="op"
                :key="`x-${op.label}`"
              ></el-option>
            </el-select>
          </el-form-item>
          <el-form-item label="Y axis: ">
            <el-select value-key="label" v-model="figureSelect.y" size="mini" placeholder="Y axis">
              <el-option
                v-for="op in tableOptions"
                :label="op.label"
                :value="op"
                :key="`y-${op.label}`"
              ></el-option>
            </el-select>
          </el-form-item>
          <el-form-item>
            <el-button
              style="margin-left: 1em;"
              size="mini"
              plain
              type="primary"
              @click="plotFigure"
            >Plot figure</el-button>
          </el-form-item>
        </el-form>
      </section>

      <section style="display:flex; flex-wrap:wrap; justify-content: space-around;">
        <div
          style="width:400px;height:300px;"
          :id="'figure-'+index"
          v-for="index in Array(allFigures.length).fill(allFigures.length).map((x,i)=> (x-1-i))"
          :key="'figure-'+index"
        ></div>
      </section>
    </el-card>
  </div>
</template>

<script>
import { mapState, mapMutations } from "vuex";
import echarts from "echarts";
var get = require("lodash/get");
let cloneDeep = require("lodash/cloneDeep");

export default {
  name: "Analyze",
  props: {},
  data() {
    return {
      currentSelected: {},
      tableOptions: [
        {
          fixed: true,
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
        { fixed: true, prop: "result.basics.Throughput", label: "Throughput" }
      ],
      figureSelect: {},
      allFigures: []
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
      const xValues = this.currentSelected.map(item => {
        return get(item, self.figureSelect.x.prop);
      });
      const yValues = this.currentSelected.map(item => {
        return get(item, self.figureSelect.y.prop);
      });
      const newOption = {
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
      };
      this.allFigures.push(cloneDeep(newOption));

      this.$nextTick(function() {
        let dom = document.getElementById(
          "figure-" + (self.allFigures.length - 1)
        );
        let newChart = echarts.init(dom);
        newChart.setOption(newOption);
      });
    }
  }
};
</script>

<style scoped>
</style>
