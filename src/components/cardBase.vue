<script setup>

defineProps(['title',])

</script>

<template>
  <el-card>
    <div class="fs5 fw-bolder">
      <span>{{ title }}</span>
    </div>
    <slot name="content"></slot>
    <!-- <template #footer>Footer content</template> -->
  </el-card>
</template>

<style scoped>
.el-card {
  --el-card-padding: 10px;
}
</style>
