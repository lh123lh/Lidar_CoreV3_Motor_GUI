<script setup>

defineProps(['title',])

</script>

<template>
  <el-card style="min-width: 350px; ">
    <div class="fw-bolder mt-n3 mb-n4">
      <el-row>
        <el-col :xs="18" :sm="18" :md="18" :lg="18" :xl="18">
          <span>{{ title }}</span>
        </el-col>
        <el-col :xs="6" :sm="6" :md="6" :lg="6" :xl="6">
          <slot name="headBtn"></slot>
        </el-col>
      </el-row>
    </div>
    <el-divider content-position="left" />
    <!-- <el-scrollbar height="200px"> -->
      <slot name="content"></slot>
    <!-- </el-scrollbar> -->
    <!-- <template #footer>Footer content</template> -->
  </el-card>
</template>
