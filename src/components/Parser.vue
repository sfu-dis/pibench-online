<!--
Copyright (c) Simon Fraser University. All rights reserved.
Licensed under the MIT license.

Authors:
Xiangpeng Hao <xiangpeng_hao@sfu.ca>
-->
<template>
  <div style="margin: 1em 2em 0 2em;">
    <h1>From PiBench std output to JSON</h1>
    <section id="editor-container">
      <el-card>
        <div
          id="container-src"
          style="width: 43vw; height: 80vh;"
        ></div>
      </el-card>
      <div>
        <el-button
          @click="convert_json"
          size="mini"
          style="height: 3em;"
        >️Convert ➡️</el-button>
      </div>
      <el-card>
        <div
          id="container-json"
          style="width: 43vw; height: 80vh;"
        >
        </div>
      </el-card>
    </section>
  </div>
</template>

<script>
import sample_text from "@/assets/sample_output.txt";
import * as monaco from "monaco-editor";
import { PiBenchData } from "pibench-parser";
export default {
  name: "Parser",
  props: {},
  data() {
    return {
      raw_editor: null,
      json_editor: null
    };
  },
  mounted() {
    this.raw_editor = monaco.editor.create(
      document.getElementById("container-src"),
      {
        value: sample_text,
        language: "text"
      }
    );

    this.json_editor = monaco.editor.create(
      document.getElementById("container-json"),
      {
        value: "",
        language: "json"
      }
    );
  },
  methods: {
    convert_json() {
      let value = this.raw_editor.getValue();
      const result = PiBenchData.from_text(value).to_js_value();
      this.json_editor.setValue(JSON.stringify(result));
      let self = this;
      setTimeout(function() {
        self.json_editor.getAction("editor.action.formatDocument").run();
      }, 10);
    }
  }
};
</script>

<style scoped>
#editor-container {
  display: flex;
  justify-content: space-between;
  /* margin: 1em; */
}
</style>
