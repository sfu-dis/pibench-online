// Copyright (c) Simon Fraser University. All rights reserved.
// Licensed under the MIT license.

// Authors:
// Xiangpeng Hao <xiangpeng_hao@sfu.ca>

<template>
  <div>
    <!-- <el-button @click="dialogVisible=true">Add Backend</el-button> -->
    <section style="display:flex;">
      <el-card class="config-zone">
        <el-form
          @submit.native.prevent
          ref="form-basic"
          :inline="true"
          size="mini"
          :model="formBasic"
          label-width="10em"
        >
          <el-form-item label="Backend">
            <el-select
              @change="changeBackend"
              v-model="selectedBackend"
              placeholder="Select Backend"
              value-key="name"
            >
              <el-option v-for="item in backends" :key="item.name" :label="item.name" :value="item"></el-option>
            </el-select>
          </el-form-item>
          <el-form-item label="Wrapper">
            <el-select v-model="formBasic.wrapper" placeholder="Select Wrapper">
              <el-option v-for="item in wrappers" :key="item" :label="item" :value="item"></el-option>
            </el-select>
          </el-form-item>
        </el-form>
        <el-form
          ref="form-config"
          :inline="true"
          size="mini"
          :model="piBenchParams"
          label-width="10em"
          @submit.native.prevent
        >
          <el-form-item label="Threads">
            <el-input-number v-model="piBenchParams.thread_cnt" :step="1" :min="1" :max="10"></el-input-number>
          </el-form-item>
          <el-form-item label="Operations">
            <el-input-number v-model="piBenchParams.op_cnt" :step="1000" :min="1"></el-input-number>
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

          <el-form-item label="Seed">
            <el-input-number v-model="piBenchParams.seed" :step="1"></el-input-number>
          </el-form-item>
          <el-form-item label="Key Prefix">
            <el-input v-model="piBenchParams.key_prefix"></el-input>
          </el-form-item>

          <el-form-item label="Use PCM">
            <el-switch v-model="piBenchParams.use_pcm"></el-switch>
          </el-form-item>

          <el-form-item label="Pool Path">
            <el-input v-model="piBenchParams.pool_path"></el-input>
          </el-form-item>

          <el-form-item label="Skip Load">
            <el-switch v-model="piBenchParams.skip_load"></el-switch>
          </el-form-item>

          <el-form-item label="Distribution">
            <el-select v-model="piBenchParams.distribution" placeholder="Distribution">
              <el-option
                v-for="item in possibleDistributions"
                :key="item"
                :label="item"
                :value="item"
              ></el-option>
            </el-select>
          </el-form-item>
          <el-form-item label="Env">
            <el-input type="textarea" v-model="env"></el-input>
          </el-form-item>
          <el-form-item>
            <el-button @click="savePreset">Save Preset</el-button>
          </el-form-item>
          <el-form-item>
            <el-button type="primary" @click="startBenchmark">Start</el-button>
          </el-form-item>
        </el-form>
      </el-card>
      <el-card class="result-zone" v-if="showResult">
        <results ref="results" v-loading="resultLoading"></results>
      </el-card>
    </section>
  </div>
</template>

<script>
import { postBenchmark, fetchServerStatus } from "@/api.js";
import { resultParser } from "@/resultParser.js";

import Results from "@/components/Results.vue";

import { mapState, mapMutations } from "vuex";

export default {
  name: "Benchmark",
  props: {},
  components: {
    results: Results
  },
  data() {
    return {
      wrappers: [],
      formBasic: {},
      possibleDistributions: ["UNIFORM", "ZIPFAN"],
      piBenchParams: {
        thread_cnt: 1,
        op_cnt: 3000000,
        load_cnt: 1000,
        read: 0,
        insert: 1,
        delete: 0,
        update: 0,
        sample_time: 100,
        scan_size: 100,
        key_size: 8,
        value_size: 8,
        key_prefix: "",
        use_pcm: false,
        seed: 42,
        skip_load: true,
        pool_size: 1024 * 1024,
        pool_path: "pool.data",
        distribution: "UNIFORM",
        latency_sampling: 0.1
      },
      env: "PMEM_IS_PMEM_FORCE=1,",
      showResult: false,
      resultLoading: true,
      serverResponse: {},
      selectedBackend: {}
    };
  },
  computed: mapState(["backends", "configPresets"]),
  methods: {
    ...mapMutations({
      addConfigPreset: "addConfigPreset"
    }),
    savePreset() {
      const data = {
        basic: this.formBasic,
        params: this.piBenchParams,
        env: this.env
      };
      this.addConfigPreset({
        name: `preset${this.configPresets.length}`,
        data: data
      });
    },
    async startBenchmark() {
      const data = {
        basic: this.formBasic,
        params: this.piBenchParams,
        env: this.env
      };
      const response = await postBenchmark(this.formBasic.backend, data);
      if (response["result"] !== "Ok") {
        this.$message("Server Error!");
        console.log(response);
        return;
      }
      this.$message("Benchmark started!");
      this.resultLoading = true;
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
        serverStatus = await fetchServerStatus(
          this.formBasic.backend,
          this.serverResponse["pid"]
        );
      } while (serverStatus["status"] === "running");

      if (serverStatus["status"] === "finished") {
        this.$message("Server finished the benchmark!");

        try {
          const cleanResults = resultParser(serverStatus["data"]);
          this.$refs.results.updateResults(cleanResults, {
            basic: this.formBasic,
            params: this.piBenchParams,
            env: this.env
          });
          this.resultLoading = false;
        } catch (error) {
          this.$message(
            "Failed to parse the result, check the console for raw result"
          );
          console.error(error);
          console.log(serverStatus);
          console.log(serverStatus["data"]);
        }
      }
    },
    changeBackend(val) {
      this.formBasic.backend = val.url;
      this.wrappers = val.wrappers;
    }
  }
};
</script>

<style scoped>
.config-zone {
  width: 45%;
  margin-right: 2em;
}

.el-form .el-select {
  width: 9.3em;
}
.el-form .el-input {
  width: 11em;
}
</style>

