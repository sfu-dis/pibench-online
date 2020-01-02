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
            <el-tag
              closable
              size="small"
              v-for="wrapper in item['wrappers']"
              :key="wrapper"
              @close="removeWrapper(index, wrapper)"
            >{{wrapper}}</el-tag>

            <el-upload
              name="wrapper"
              action="http://dbserver.haoxp.xyz:8000/upload_wrapper/"
              :show-file-list="false"
              :on-success="updateBackend(index)"
              :on-error="uploadFailed"
            >
              <el-button plain>Upload Wrapper</el-button>
            </el-upload>
          </el-form-item>

          <el-form-item>
            <el-button plain type="danger" @click="deleteBackend(index)">Delete</el-button>
          </el-form-item>
        </el-form>
      </section>
      <el-button type="primary" size="mini" @click="dialogVisible=true">Add New Backend</el-button>

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
    </el-card>
    <el-card>
      <h3>Cache</h3>
      <el-button type="danger" size="mini" plain @click="cleanStorage">Reset local storage</el-button>
    </el-card>
  </div>
</template>

<script>
import { fetchInstanceInfo, removeWrapperApi } from "@/api.js";
import { mapState, mapMutations } from "vuex";

export default {
  name: "Settings",
  props: {},
  computed: mapState(["backends"]),
  data() {
    return {
      dialogVisible: false,
      backendUrlInput: "http://dbserver.haoxp.xyz:8000"
    };
  },
  methods: {
    ...mapMutations({
      addBackendStore: "addBackend",
      deleteBackendStore: "deleteBackend",
      updateBackendStore: "updateBackend"
    }),
    cleanStorage() {
      localStorage.setItem("vuex", "");
      location.reload();
    },
    async addBackend() {
      try {
        const data = await fetchInstanceInfo(this.backendUrlInput);
        this.addBackendStore(data);
        this.dialogVisible = false;
      } catch {
        this.$message("Invalid backend!");
      }
    },
    updateBackend(index) {
      return () => {
        try {
          let self = this;
          const targetBackend = this.backends[index];
          fetchInstanceInfo(targetBackend.url).then(data => {
            self.updateBackendStore({ index, backend: data });
          });
        } catch {
          this.$message("Invalid backend!");
        }
      };
    },
    async removeWrapper(index, wrapper) {
      const data = await removeWrapperApi(this.backends[index].url, wrapper);
      if (data["result"] === "ok") {
        this.updateBackend(index)();
      } else {
        this.$message("Delete failed!");
      }
    },
    async uploadFailed() {
      this.$message("Upload failed!");
    },
    deleteBackend(index) {
      this.deleteBackendStore(index);
    }
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
  margin: 0 1em 1em 0;
  border: 1px solid #ebeef5;
  padding: 1em;
  min-width: 30%;
}
.el-card {
  margin-bottom: 1em;
}
</style>

