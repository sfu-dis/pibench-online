<template>
  <div id="app">
    <h1>PiBench Online</h1>
    <el-button @click="testParser">test</el-button>

    <el-form ref="form-basic" :inline="true" :model="formBasic" label-width="10em">
      <el-form-item>
        <el-button @click="dialogVisible=true">Add Backend</el-button>
      </el-form-item>
      <el-form-item label="PiBench Backend">
        <el-select @change="changeBackend" :value="formBasic.backend" placeholder="Select Backend">
          <el-option v-for="item in backends" :key="item.url" :label="item.url" :value="item"></el-option>
        </el-select>
      </el-form-item>
      <el-form-item label="Wrapper">
        <el-select v-model="formBasic.wrapper" placeholder="Select Wrapper">
          <el-option v-for="item in wrappers" :key="item" :label="item" :value="item"></el-option>
        </el-select>
      </el-form-item>
    </el-form>

    <el-dialog title="Add Backend" :visible.sync="dialogVisible" width="40%">
      <el-form :inline="true">
        <el-form-item label="PiBench Backend URL:">
          <el-input v-model="backendUrlInput"></el-input>
        </el-form-item>
      </el-form>
      <span slot="footer" class="dialog-footer">
        <el-button @click="dialogVisible = false">Cancel</el-button>
        <el-button type="primary" @click="addBackend">Confirm</el-button>
      </span>
    </el-dialog>

    <el-form ref="form-config" :model="piBenchParams" label-width="10em">
      <el-form-item label="Threads">
        <el-input-number v-model="piBenchParams.thread_cnt" :step="1" :min="1" :max="10"></el-input-number>
      </el-form-item>
      <el-form-item label="Operations">
        <el-input-number v-model="piBenchParams.op_cnt" :step="1" :min="1" :max="10"></el-input-number>
      </el-form-item>
      <el-form-item label="Load Count">
        <el-input-number v-model="piBenchParams.load_cnt" :step="1" :min="1" :max="10"></el-input-number>
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

      <el-form-item label="Latency Sampling">
        <el-input-number v-model="piBenchParams.latency_sampling" :step="0.1" :min="0" :max="1"></el-input-number>
      </el-form-item>
      <el-form-item label="Sample Time (ms)">
        <el-input-number v-model="piBenchParams.sample_time" :step="100" :min="100" :max="2000"></el-input-number>
      </el-form-item>

      <el-form-item label="Key Prefix">
        <el-input v-model="piBenchParams.key_prefix"></el-input>
      </el-form-item>

      <el-form-item label="Key Size">
        <el-input-number v-model="piBenchParams.key_size" :min="4" :step="1"></el-input-number>
      </el-form-item>

      <el-form-item label="Value Size">
        <el-input-number v-model="piBenchParams.value_size" :min="4" :step="1"></el-input-number>
      </el-form-item>
      <el-form-item label="Scan Size">
        <el-input-number v-model="piBenchParams.scan_size" :min="1" :step="1"></el-input-number>
      </el-form-item>

      <el-form-item label="Pool Size">
        <el-input-number v-model="piBenchParams.pool_size" :min="1" :step="1"></el-input-number>
      </el-form-item>

      <el-form-item label="Pool Path">
        <el-input v-model="piBenchParams.pool_path"></el-input>
      </el-form-item>

      <el-form-item label="Distribution">
        <el-select v-model="piBenchParams.distribution" placeholder="Distribution">
          <el-option v-for="item in possibleDistributions" :key="item" :label="item" :value="item"></el-option>
        </el-select>
      </el-form-item>
      <el-form-item label="Seed">
        <el-input-number v-model="piBenchParams.seed" :step="1"></el-input-number>
      </el-form-item>
      <el-form-item label="Use PCM">
        <el-switch v-model="piBenchParams.use_pcm"></el-switch>
      </el-form-item>
      <el-form-item label="Skip Load">
        <el-switch v-model="piBenchParams.skip_load"></el-switch>
      </el-form-item>
      <el-form-item label="Env">
        <el-input type="textarea" v-model="env"></el-input>
      </el-form-item>
      <el-form-item>
        <el-button type="primary" @click="startBenchmark">Start</el-button>
      </el-form-item>
    </el-form>

    <results ref="results" v-if="showResult"></results>
  </div>
</template>

<script>
import { fetchInstanceInfo, postBenchmark, fetchServerStatus } from "@/api.js";
import { resultParser } from "@/resultParser.js";

import Results from "@/components/Results.vue";

export default {
  name: "app",
  components: {
    results: Results
  },
  data() {
    return {
      wrappers: [],
      backends: [],
      formBasic: {},
      possibleDistributions: ["UNIFROM", "ZIPFAN"],
      dialogVisible: false,
      backendUrlInput: "http://localhost:8000",
      piBenchParams: {
        thread_cnt: 1,
        op_cnt: 1000,
        load_cnt: 1000,
        read: 0.5,
        insert: 0.5,
        delete: 0,
        update: 0,
        sample_time: 500,
        scan_size: 100,
        key_size: 8,
        value_size: 8,
        key_prefix: "",
        use_pcm: false,
        seed: 42,
        skip_load: true,
        pool_size: 1024 * 1024,
        pool_path: "pool.data",
        distribution: "UNIFROM",
        latency_sampling: 0.1
      },
      env: "PMEM_IS_PMEM_FORCE=1,",
      showResult: false,
      resultLoading: true,
      serverResponse: {}
    };
  },
  mounted() {},
  methods: {
    testParser() {
      const text =
        "Environment:\n\tTime: Fri Dec 20 06:39:39 2019\n\tCPU: 8 * Intel(R) Core(TM) i7-8550U CPU @ 1.80GHz\n\tCPU Cache: 8192 KB\n\tKernel: Linux 4.19.84-microsoft-standard\nBenchmark Options:\n\tTarget: /home/hao/pibench/release/libstlmap_wrapper.so\n\t# Records: 1000000\n\t# Operations: 1000000\n\t# Threads: 1\n\tSampling: 1000 ms\n\tLatency: 0\n\tKey prefix: \n\tKey size: 4\n\tValue size: 4\n\tRandom seed: 1729\n\tKey distribution: UNIFORM\n\tScan size: 100\n\tOperations ratio:\n\t\tRead: 0\n\t\tInsert: 1\n\t\tUpdate: 0\n\t\tDelete: 0\n\t\tScan: 0\nOverview:\n\tLoad time: 905.411 milliseconds\n\tRun time: 1409.6847 milliseconds\n\tThroughput: 709378.5000 ops/s\nSamples:\n\t739068\n\t260932\n";
      console.log(resultParser(text));
    },
    async addBackend() {
      try {
        const data = await fetchInstanceInfo(this.backendUrlInput);
        this.backends.push(data);
        this.dialogVisible = false;
      } catch {
        this.$message("Invalid backend!");
      }
    },
    async startBenchmark() {
      const data = {
        basic: this.formBasic,
        params: this.piBenchParams,
        env: this.env
      };
      const response = await postBenchmark(data);
      if (response["result"] !== "Ok") {
        this.$message("Server Error!");
        console.log(response);
        return;
      }
      this.$message("Benchmark started!");
      this.serverResponse = response;
      await this.checkServerStatus();
    },
    async checkServerStatus() {
      if (!this.serverResponse["pid"]) {
        return;
      }
      this.showResult = true;
      this.resultLoading = true;
      const wait = time => new Promise(tick => setTimeout(tick, time));
      let serverStatus = { status: "running" };
      do {
        await wait(1000);
        serverStatus = await fetchServerStatus(this.serverResponse["pid"]);
      } while (serverStatus["status"] === "running");

      if (serverStatus["status"] === "finished") {
        this.$message("Server finished the benchmark!");

        const cleanResults = resultParser(serverStatus["data"]);
        this.$refs.results.updateResults(cleanResults);
      }
    },
    changeBackend(val) {
      this.formBasic.backend = val.url;
      this.wrappers = val.wrappers;
    }
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
