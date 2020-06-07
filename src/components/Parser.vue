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
let example_text = `Environment:
	Time: Sat May  9 13:59:11 2020
	CPU: 96 * Intel(R) Xeon(R) Gold 6252 CPU @ 2.10GHz
	CPU Cache: 36608 KB
	Kernel: Linux 5.5.4-arch1-1
Benchmark Options:
	Target: /home/hao/coding/bztree/release/libbztree_pibench_wrapper.so
	# Records: 10000000
	# Operations: 10000000
	# Threads: 4
	Sampling: 1000 ms
	Latency: 0.2
	Key prefix: 
	Key size: 8
	Value size: 8
	Random seed: 1729
	Key distribution: UNIFORM
	Scan size: 100
	Operations ratio:
		Read: 0.2
		Insert: 0.8
		Update: 0
		Delete: 0
		Scan: 0
creating new tree on pool.
IBRS and IBPB supported  : yes
STIBP supported          : yes
Spec arch caps supported : yes
IBRS enabled in the kernel   : yes
STIBP enabled in the kernel  : no
The processor is not susceptible to Rogue Data Cache Load: yes
The processor supports enhanced IBRS                     : yes
Overview:
	Load time: 88565.1 milliseconds
	Run time: 20560.1523 milliseconds
	Throughput: 486377.7188 ops/s
PCM Metrics:
	L3 misses: 125130829
	DRAM Reads (bytes): 1841537216
	DRAM Writes (bytes): 1544076736
	NVM Reads (bytes): 57978386048
	NVM Writes (bytes): 11435547648
Samples:
	4742342
	468784
	459595
	438294
	476701
	470345
	478505
	479146
	494873
	507295
	495827
	504435
	516082
	521672
	519169
	507680
	499611
	491987
	483556
	492502
	219939
Latencies (994788 operations observed):
	min: 868
	50%: 7614
	90%: 9590
	99%: 44825
	99.9%: 54204
	99.99%: 84754
	99.999%: 6236713
	max: 6721486

`;

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
        value: example_text,
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
