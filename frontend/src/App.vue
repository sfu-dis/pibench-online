<!--
Copyright (c) Simon Fraser University. All rights reserved.
Licensed under the MIT license.

Authors:
Xiangpeng Hao <xiangpeng_hao@sfu.ca>
-->

<template>
  <div id="app">
    <el-menu
      :default-active="menuActiveIndex"
      :router="true"
      mode="horizontal"
    >
      <el-menu-item @click="dialogVisible=true">PiBench Online</el-menu-item>
      <el-menu-item index="benchmark">Benchmark</el-menu-item>
      <el-menu-item index="analyze">Analyze</el-menu-item>
      <el-menu-item index="parser">Parser</el-menu-item>
      <el-menu-item index="settings">Settings</el-menu-item>
    </el-menu>

    <el-dialog
      class="dialog-content"
      title="PiBench Online"
      :visible.sync="dialogVisible"
      width="50%"
    >
      <h3>What is PiBench Online?</h3>
      <p>PiBench Online is an interactive system for benchmarking PM indexes in a fair and reproducible manner, it is based on PiBench, the first unified, highly customizable benchmarking framework for PM indexes.</p>
      <p>PiBench Online enables push-button evaluation of PM indexes: the user can upload a shared library that implements the index, set parameters to run customized benchmarks, and analyze results interactively, all through an easy-to-use web interface.</p>
      <p>
        We have made PiBench Online available at
        <a>http://pibench.org</a> and hope it can promote fair comparison and reproducibility in future PM index research.
      </p>
      <h3>About Us</h3>
      <p>
        PiBench Online is built by:
        <a href="https://haoxp.xyz">Xiangpeng Hao</a>,
        <a href="https://www2.cs.sfu.ca/~tzwang/">Tianzheng Wang</a> (
        <a href="http://systems.cs.sfu.ca/">Systems Group</a>, Simon Fraser University), Lucas Lersch (TU Dresden &#38; SAP SE) and Ismail Oukid (Snowflake Computing)
      </p>
      <p>
        Read our paper:
        <a href="http://www.vldb.org/pvldb/vol13/p574-lersch.pdf">Evaluating Persistent Memory Range Indexes</a>
      </p>

      <p>
        Contribute to PiBench:
        <a href="https://github.com/wangtzh/pibench">PiBench GitHub</a>
      </p>

      <p>
        Discussion:
        <a href="https://groups.google.com/forum/#!forum/pibench">Google Group</a>
      </p>
      <p>
        Request PiBench resources:
        <a href="mailto:support@pibench.org">support@pibench.org</a>
      </p>
      <p></p>
      <span slot="footer">
        <el-button @click="dontShowAgain">Don't show again</el-button>
        <el-button
          type="primary"
          @click="dialogVisible = false"
        >Ok</el-button>
      </span>
    </el-dialog>

    <div style="margin: 0.5em;">
      <keep-alive>
        <router-view></router-view>
      </keep-alive>
    </div>
  </div>
</template>

<script>
import { mapState, mapMutations } from "vuex";
export default {
  name: "app",
  computed: {
    ...mapState(["showAbout"])
  },
  data() {
    return {
      menuActiveIndex: "benchmark",
      dialogVisible: true
    };
  },
  created() {
    this.dialogVisible = this.showAbout;
  },
  methods: {
    ...mapMutations(["setShowAbout"]),
    dontShowAgain() {
      this.setShowAbout(false);
      this.dialogVisible = false;
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
body {
  margin: 0;
}
.dialog-content .el-dialog__body {
  padding-top: 1em;
}
</style>

<style scoped>
.dialog-content p {
  line-height: 1.7em;
  margin-top: 0.5em;
}

.dialog-content h3 {
  font-weight: 600;
  margin-bottom: 0em;
  margin-top: 0em;
}
</style>
