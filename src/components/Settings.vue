<template>
  <div>
    <el-card>
      <h3>Backends</h3>
      <section style="display: flex;">
        <div :key="`backend-${index}`" v-for="(item,index) in backends" class="form-section">
          <div style=" z-index: 999">
            <el-button
              plain
              type="text"
              style="float: right; padding: 3px 0;"
              @click="deleteBackend(index)"
            >Delete</el-button>
          </div>
          <el-form
            style="z-index:-1;"
            size="mini"
            label-position="right"
            @submit.native.prevent
            label-width="8em"
          >
            <el-form-item label="Name:">{{item['name']}}</el-form-item>
            <el-form-item label="URL:">{{item['url']}}</el-form-item>

            <el-form-item label="Has PM:">{{item['has_pm']}}</el-form-item>
            <el-form-item label="Core Count:">{{item['core_cnt']}}</el-form-item>
            <el-form-item label="Socket Count:">{{item['socket_cnt']}}</el-form-item>
            <el-form-item label="Wrappers:">
              <el-tag
                closable
                size="small"
                v-for="wrapper in item['wrappers']"
                :key="wrapper"
                @close="removeWrapper(index, wrapper)"
              >{{wrapper}}</el-tag>
              <el-upload
                name="wrapper"
                :action="`${item.url}/upload_wrapper/`"
                :show-file-list="false"
                :on-success="updateBackend(index)"
                :on-error="uploadFailed"
              >
                <el-button plain>Upload</el-button>
              </el-upload>
            </el-form-item>
          </el-form>
        </div>
      </section>
      <el-button type="primary" size="mini" @click="dialogVisible=true">Add New Backend</el-button>

      <el-dialog title="Add New Backend" :visible.sync="dialogVisible" width="35%">
        <el-form inline label-width="10em">
          <el-form-item label="Name:">
            <el-input v-model="newBackendConfig.name"></el-input>
          </el-form-item>
          <el-form-item label="Backend URL:">
            <el-input v-model="newBackendConfig.url"></el-input>
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
      newBackendConfig: {
        name: "home-server",
        url: "http://home.haoxp.xyz:7000"
      }
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
        const data = await fetchInstanceInfo(this.newBackendConfig.url);
        this.addBackendStore({
          ...data,
          url: this.newBackendConfig.url,
          name: this.newBackendConfig.name
        });
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
            const newData = {
              ...targetBackend,
              ...data
            };
            self.updateBackendStore({ index, backend: newData});
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
section .el-input {
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
.el-form-item {
  margin-bottom: 0.5em;
}
</style>

