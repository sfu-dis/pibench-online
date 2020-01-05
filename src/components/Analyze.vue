<template>
  <div>
    <el-card>
      <h1>All Benchmark Results</h1>
      <el-table :data="benchmarkResults" @selection-change="handleSelectionChange">
        <el-table-column fixed type="selection" width="55"></el-table-column>
        <el-table-column fixed prop="param.basic.wrapper" label="Wrapper"></el-table-column>
        <el-table-column prop="param.params.thread_cnt" label="Thread"></el-table-column>
        <el-table-column prop="param.params.distribution" label="Distribution"></el-table-column>
        <el-table-column prop="param.params.read" label="Read"></el-table-column>
        <el-table-column prop="param.params.insert" label="Insert"></el-table-column>
        <el-table-column prop="param.params.update" label="Update"></el-table-column>
        <el-table-column prop="param.params.delete" label="Delete"></el-table-column>
        <el-table-column prop="param.params.key_size" label="Key Size"></el-table-column>
        <el-table-column prop="param.params.value_size" label="Value Size"></el-table-column>
        <el-table-column prop="param.params.op_cnt" label="OP Count"></el-table-column>
        <el-table-column prop="result.basics.Throughput" label="Throughput"></el-table-column>
      </el-table>
      <section style="margin-top:1em;">
        <el-button @click="deleteSelected" size="mini" plain type="danger">Delete Selected</el-button>
      </section>
    </el-card>
  </div>
</template>

<script>
import { mapState, mapMutations } from "vuex";

export default {
  name: "Analyze",
  props: {},
  data() {
    return { currentSelected: {} };
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
    }
  }
};
</script>

<style scoped>
</style>
