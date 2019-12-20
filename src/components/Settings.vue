<template>
  <div>
    <el-card>
      <h3>Backends</h3>
      <section style="display: flex;">
        <el-form
          size="mini"
          :key="`backend-${index}`"
          v-for="(item,index) in backends"
          label-position="right"
          @submit.native.prevent
          class="form-section"
        >
          <el-form-item label="URL:">{{item['url']}}</el-form-item>
          <el-form-item label="Has PM:">
            <el-switch :disabled="true" :value="item['has_pm']"></el-switch>
          </el-form-item>
          <el-form-item label="Core Count:">
            <el-input :value="item['core_cnt']" :disabled="true"></el-input>
          </el-form-item>
          <el-form-item label="Socket Count:">
            <el-input :value="item['socket_cnt']" :disabled="true"></el-input>
          </el-form-item>
          <el-form-item label="Avaliable Wrappers:">
            <el-tag v-for="wrapper in item['wrappers']" :key="wrapper">{{wrapper}}</el-tag>
          </el-form-item>
        </el-form>
      </section>
    </el-card>
  </div>
</template>

<script>
import { mapState, mapMutations } from "vuex";

export default {
  name: "Settings",
  props: {},
  computed: mapState(["backends"]),
  methods: {
    ...mapMutations({
      addBackendStore: "addBackend"
    })
  }
};
</script>

<style scoped>
.el-input {
  width: 7em;
}
.el-tag {
  margin-right: 1em;
}
.form-section {
  margin: 1em;
  border: 1px solid #ebeef5;
  padding: 1em;
  min-width: 30%;
}
</style>

