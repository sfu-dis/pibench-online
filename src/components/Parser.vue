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
    <section id="editor-container">
      <div
        id="container-src"
        style="height:50em; width: 45%;"
      ></div>
      <div
        id="container-json"
        style="height: 50em; width: 45%;"
      >
      </div>
    </section>
  </div>
</template>

<script>
import * as monaco from "monaco-editor";
export default {
  name: "Parser",
  props: {},
  mounted() {
    console.log("test");

    monaco.editor.create(document.getElementById("container-src"), {
      value: "function hello() {\n\talert('Hello world!');\n}",
      language: "javascript"
    });

    monaco.editor.create(document.getElementById("container-json"), {
      value: "function hello() {\n\talert('Hello world!');\n}",
      language: "javascript"
    });
  }
};
</script>

<style scoped>
#editor-container {
  display: flex;
  justify-content: space-between;
}
</style>
