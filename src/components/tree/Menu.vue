<script setup>
import Settings from "./../Settings.vue";
import NewProject from "./../NewProject.vue";
</script>
<template>
  <el-container>
    <div class="operations">
      <el-row :gutter="0">
        <el-col :span="12" @click="newProject">New</el-col>
        <el-col :span="12" @click="openProject">Open</el-col>
      </el-row>
    </div>
    <div class="settings" @click="settingVisible = true">
      <el-icon><Setting /> </el-icon>
    </div>
  </el-container>
  <Settings
    v-if="settingVisible"
    :visible="settingVisible"
    @update="updateSetting"
  />
  <NewProject
    v-if="newProjectVisible"
    :visible="newProjectVisible"
    @close="newProjectVisible = false"
    @open:project="openProject"
  />
</template>
<style scoped>
.el-container {
  display: flex;
  .operations {
    width: 100%;
    .el-col {
      height: 30px;
      line-height: 30px;
      text-align: center;
      font-size: 12px;
    }
  }
  .settings {
    width: 40px;
    display: flex;
    justify-content: center;
    align-items: center;
  }
  .operations .el-col,
  .settings {
    background-color: var(--Secondary-Fill);
    cursor: pointer;
  }
  .operations .el-col:hover,
  .settings:hover {
    color: var(--Secondary-Color);
  }
}
</style>
