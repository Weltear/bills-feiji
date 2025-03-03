<script setup>
import { ref } from "vue";
import AccsTable from "../component/AccsTable.vue";
</script>

<template>
    <div class="inquire-view">
        <el-container>
            <el-header style="font-size: 32px;">日常开销</el-header>
            <el-main>
                <accs-table />
            </el-main>
        </el-container>
    </div>
</template>

<style scoped>

.inquire-dropdown-link{
    cursor: pointer;
    color: var(--el-color-primary);
    display: flex;
    align-items: center;
    margin-bottom: 8px;
}

.dropdown-button-text {
    font-size: 14px;
}
</style>