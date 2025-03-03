<script setup>
import { ref } from "vue";
import SettingCommon from "../component/SettingCommon.vue";

const activeName = ref('first');

function settingTabChanged () {

}
</script>

<template>
    <div class="setting-view">
        <el-container>
            <el-header style="font-size: 32px;">设置</el-header>

            <el-main>
                <el-tabs 
                    v-model="activeName" 
                    class="setting-tabs" 
                    @tab-click="settingTabChanged"
                    type="">
                    <el-tab-pane label="常规" name="first"><SettingCommon /></el-tab-pane>
                    <el-tab-pane label="关于" name="second">关于</el-tab-pane>
                </el-tabs>
            </el-main>
        </el-container>
    </div>
</template>

<style scoped>
.setting-tabs :deep(.el-tabs__item) {
    font-size: 20px;
    font-weight: 600;
    height: 40px;
    width: 100px;
}

.setting-tabs :deep(.el-tabs__header) {
    height: 40px;
}
</style>