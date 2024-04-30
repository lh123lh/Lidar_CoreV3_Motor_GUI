<script setup>

defineProps(['title',])

</script>

<template>
  <el-container class="me-n3 ms-n3">
    <el-header class="ms-1" style="background-color: white; height: 25px;">
      <el-row :gutter="20" class="mt-0">
        <el-col :span="6">
          <span class="fs-2 fw-bolder">
            {{ title }}
          </span>
        </el-col>
        <el-col :span="12" :offset="6" style="text-align: right;">
          <slot name="header"></slot>
        </el-col>
      </el-row>
    </el-header>
    <el-main style="height: 100vh" class="mt-n3">
      <el-scrollbar view-style="overflow-x: hidden;">
        <slot></slot>
      </el-scrollbar>
    </el-main>
  </el-container>
</template>
