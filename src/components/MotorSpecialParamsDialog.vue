<script setup>
import { ref } from 'vue'
import cmds from '../api/cmds';
import { MotorParams } from '../stores/motorState';

const params = MotorParams();
const visable = defineModel();

const loading = ref(false);
const uploading = ref(false);

async function get_motor_special_params() {
  loading.value = true;
  await cmds.cmd_get_motor_special_params()
    .then((data) => {
      console.log(data);
      params.featureParams.poles = data.feature_param.poles;
      params.featureParams.rs_ohm = data.feature_param.rs_ohm.toFixed(8);
      params.featureParams.ls_d = data.feature_param.ls_d.toFixed(10);
      params.featureParams.ls_q = data.feature_param.ls_q.toFixed(10);
      params.featureParams.rated_flux = data.feature_param.rated_flux.toFixed(10);
      params.featureParams.res_est_current = data.feature_param.res_est_current;
      params.featureParams.ind_est_current = data.feature_param.ind_est_current;
      params.featureParams.max_current = data.feature_param.max_current;
      params.featureParams.flux_exc_freq = data.feature_param.flux_exc_freq;
      params.featureParams.wbp_kgm2 = data.feature_param.wbp_kgm2.toFixed(2);
      params.featureParams.rated_voltage = data.feature_param.rated_voltage;

      params.startupParams.flux_current = data.startup_param.flux_current;
      params.startupParams.align_current = data.startup_param.align_current;
      params.startupParams.startup_current = data.startup_param.startup_current;
      params.startupParams.torque_current = data.startup_param.torque_current;
      params.startupParams.speed_start = data.startup_param.speed_start;
      params.startupParams.speed_force = data.startup_param.speed_force;

      params.faultChkParams.over_voltage = data.fault_check_param.over_voltage;
      params.faultChkParams.under_voltage = data.fault_check_param.under_voltage;
      params.faultChkParams.over_load_power = data.fault_check_param.over_load_power;
      params.faultChkParams.stall_current = data.fault_check_param.stall_current;
      params.faultChkParams.fault_ckeck_current = data.fault_check_param.fault_ckeck_current.toFixed(2);
      params.faultChkParams.fail_speed_max = data.fault_check_param.fail_speed_max;
      params.faultChkParams.fail_speed_min = data.fault_check_param.fail_speed_min;
      loading.value = false;
    })
}

</script>

<template>
  <el-dialog v-model="visable" title="电机特性参数设置" width="650">
    <el-form :model="params" ref="form" label-width="auto" label-position="right" :inline="false">

      <el-scrollbar max-height="15.0rem" view-style="overflow-x: hidden;">
        <span class="fs4 fw-bolder">特性参数 :</span>
        <el-row :gutter="20">
          <el-col :span="12">
            <el-form-item label="磁极对数">
              <el-input v-model="params.featureParams.poles"></el-input>
            </el-form-item></el-col>
          <el-col :span="12">
            <el-form-item label="Rs (Ohm)">
              <el-input v-model="params.featureParams.rs_ohm"></el-input>
            </el-form-item></el-col>
        </el-row>

        <el-row :gutter="20" class="mt-n3">
          <el-col :span="12">
            <el-form-item label="Ls_D">
              <el-input v-model="params.featureParams.ls_d"></el-input>
            </el-form-item></el-col>
          <el-col :span="12">
            <el-form-item label="Ls_Q">
              <el-input v-model="params.featureParams.ls_q"></el-input>
            </el-form-item></el-col>
        </el-row>

        <el-row :gutter="20" class="mt-n3">
          <el-col :span="12">
            <el-form-item label="Flux (VpHz)">
              <el-input v-model="params.featureParams.rated_flux"></el-input>
            </el-form-item></el-col>
          <el-col :span="12">
            <el-form-item label="Wb P (Kgm2)">
              <el-input v-model="params.featureParams.wbp_kgm2"></el-input>
            </el-form-item></el-col>
        </el-row>

        <el-row :gutter="20" class="mt-n3">
          <el-col :span="12">
            <el-form-item label="额定电压">
              <el-input v-model="params.featureParams.rated_voltage"></el-input>
            </el-form-item></el-col>
          <el-col :span="12">
            <el-form-item label="最大电流">
              <el-input v-model="params.featureParams.max_current"></el-input>
            </el-form-item></el-col>
        </el-row>

        <el-form-item label="Rs估算电流" class="mt-n3">
          <el-col :span="10">
            <el-input v-model="params.featureParams.ind_est_current"></el-input>
          </el-col>
          <el-col :span="4" style="text-align: center;">
            <span class="text-gray-500">-</span>
          </el-col>
          <el-col :span="10">
            <el-input v-model="params.featureParams.res_est_current"></el-input>
          </el-col>
        </el-form-item>

        <span class="fs4 fw-bolder">启动参数 :</span>
        <el-row :gutter="20">
          <el-col :span="12">
            <el-form-item label="开环电流">
              <el-input v-model="params.startupParams.flux_current"></el-input>
            </el-form-item></el-col>
          <el-col :span="12">
            <el-form-item label="对准电流">
              <el-input v-model="params.startupParams.align_current"></el-input>
            </el-form-item></el-col>
        </el-row>

        <el-row :gutter="20" class="mt-n3">
          <el-col :span="12">
            <el-form-item label="启动电流">
              <el-input v-model="params.startupParams.startup_current"></el-input>
            </el-form-item></el-col>
          <el-col :span="12">
            <el-form-item label="转矩启动电流">
              <el-input v-model="params.startupParams.torque_current"></el-input>
            </el-form-item></el-col>
        </el-row>

        <el-row :gutter="20" class="mt-n3">
          <el-col :span="12">
            <el-form-item label="启动转速阈值">
              <el-input v-model="params.startupParams.speed_start"></el-input>
            </el-form-item></el-col>
          <el-col :span="12">
            <el-form-item label="开环启动阈值">
              <el-input v-model="params.startupParams.speed_force"></el-input>
            </el-form-item></el-col>
        </el-row>

        <span class="fs4 fw-bolder">故障检测参数 :</span>
        <el-row :gutter="20">
          <el-col :span="12">
            <el-form-item label="过压阈值">
              <el-input v-model="params.faultChkParams.over_voltage"></el-input>
            </el-form-item></el-col>
          <el-col :span="12">
            <el-form-item label="欠压阈值">
              <el-input v-model="params.faultChkParams.under_voltage"></el-input>
            </el-form-item></el-col>
        </el-row>

        <el-row :gutter="20" class="mt-n3">
          <el-col :span="12">
            <el-form-item label="过载阈值">
              <el-input v-model="params.faultChkParams.over_load_power"></el-input>
            </el-form-item></el-col>
          <el-col :span="12">
            <el-form-item label="堵转电流">
              <el-input v-model="params.faultChkParams.stall_current"></el-input>
            </el-form-item></el-col>
        </el-row>

        <el-row :gutter="20" class="mt-n3">
          <el-col :span="12">
            <el-form-item label="超速阈值">
              <el-input v-model="params.faultChkParams.fail_speed_max"></el-input>
            </el-form-item></el-col>
          <el-col :span="12">
            <el-form-item label="最小速度阈值">
              <el-input v-model="params.faultChkParams.fail_speed_min"></el-input>
            </el-form-item></el-col>
        </el-row>

        <el-row :gutter="20" class="mt-n3">
          <el-col :span="12">
            <el-form-item label="故障检测电流">
              <el-input v-model="params.faultChkParams.fault_ckeck_current"></el-input>
            </el-form-item></el-col>
        </el-row>

      </el-scrollbar>

      <el-form-item class="mt-3 mb-n1">
        <el-button v-if="!loading" @click="get_motor_special_params" type="success" plain>刷新</el-button>
        <el-button v-else type="success" loading plain>刷新中</el-button>

        <el-button type="danger" plain>设置</el-button>
      </el-form-item>
    </el-form>

  </el-dialog>
</template>
