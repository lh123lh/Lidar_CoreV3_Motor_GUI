<script setup>

defineProps(['title',])

</script>

<template>
  <el-container>
    <el-header style="background-color: white; height: 30px;">
      <el-row :gutter="20">
        <span class="fs-2 fw-bolder">
          {{ title }}
        </span>
      </el-row>
    </el-header>
    <el-main style="height: 100vh">
      <slot></slot>
    </el-main>
  </el-container>
</template>

<style scoped>
.el-main {
  --el-main-padding: 5px;
}
</style>
