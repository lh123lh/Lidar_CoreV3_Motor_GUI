<script setup>
import { ref } from 'vue'
import PageBase from '../components/PageBase.vue';
import cardBase from '../components/cardBase.vue';
import { useMotorStore } from '../stores/motorState';
import MotorSpecialParamsDialog from '../components/MotorSpecialParamsDialog.vue';

const status = useMotorStore();
const motorParamDialog = ref(false);

</script>

<template>
  <PageBase title="设置">
    <el-row :gutter="5">
      <el-col :xs="12" :sm="12" :md="12" :lg="12" :xl="12">
        <cardBase title="电机设置" class="mt-1">
          <template #content>
            <el-row :gutter="20" class="mt-1">
              <el-col :span="18">
                电机特性参数设置
              </el-col>
              <el-col :span="6" class=" me-n3 ps-9">
                <el-link :underline="false" :disabled=!status.isConnected class="mb-1" @click="motorParamDialog = true">
                  <el-icon :size="16">
                    <SvgIcon iconName=icon-arrow-right />
                  </el-icon>
                </el-link>
              </el-col>
            </el-row>

          </template>
        </cardBase>
      </el-col>
      <el-col :xs="12" :sm="12" :md="12" :lg="12" :xl="12"></el-col>
    </el-row>


  </PageBase>

  <MotorSpecialParamsDialog v-model="motorParamDialog" />
</template>

<style scoped></style>
