<template>
  <div id="app">
    <el-menu :default-active="menuActiveIndex" router="true" mode="horizontal">
      <el-menu-item>PiBench Online</el-menu-item>
      <el-menu-item index="benchmark">Benchmark</el-menu-item>
      <el-menu-item index="settings">Settings</el-menu-item>
    </el-menu>

    <div style="margin: 0.5em;">
      <router-view></router-view>
    </div>
  </div>
</template>

<script>
export default {
  name: "app",
  data() {
    return {
      menuActiveIndex: "benchmark"
    };
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
</style>

<style scoped>
</style>
