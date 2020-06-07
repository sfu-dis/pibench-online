<!--
Copyright (c) Simon Fraser University. All rights reserved.
Licensed under the MIT license.

Authors:
Xiangpeng Hao <xiangpeng_hao@sfu.ca>
-->
<template>
  <div>
    <h1>PiBench Parser</h1>
    <div>Parse the Pibench text result to JSON format</div>
    <el-button
      @click="convert_json"
      size="mini"
    >Convert</el-button>
    <section id="editor-container">
      <div
        id="container-src"
        style="height:60em; width: 45%;"
      ></div>
      <div
        id="container-json"
        style="height: 60em; width: 45%;"
      >
      </div>
    </section>
  </div>
</template>

<script>
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
    console.log("test");

    this.raw_editor = monaco.editor.create(
      document.getElementById("container-src"),
      {
        value: "",
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
      console.log("click convertasdf");
      let value = this.raw_editor.getValue();
      console.log(value);
      const result = PiBenchData.from_text(value).to_js_value();
      console.log(result);
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
  margin: 1em;
}
</style>
