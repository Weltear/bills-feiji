<script setup>
import { invoke } from '@tauri-apps/api/core';
import { ref } from 'vue';
import { config, inKindFilters, outKindFilters } from '../main';
import { ElNotification } from 'element-plus';

const expect_out = ref(config.value.expect_out);
// 标签操作索引
const tagEditId = ref(null);
const editing = ref(false);
const inTagDialog = ref(false);
const outTagDialog = ref(false);
// 标签备份
var back;
// 设置备份
var configBack;

// 收入弹出窗
function inTagBegin() {
    // 首先备份收入标签
    configBack = JSON.parse(JSON.stringify(inKindFilters.value));
    inTagDialog.value = true;
}

// 支出弹出窗
function outTagBegin() {
    // 首先备份收入标签
    configBack = JSON.parse(JSON.stringify(outKindFilters.value));
    outTagDialog.value = true;
}

function inTagDelete(index) {
    inKindFilters.value.splice(index, 1);

    // 若移除的是正在编辑中的栏目，移除编辑状态
    if ((index === inKindFilters.value.length) && editing.value) {
        tagEditId.value = null;
        editing.value = false;
    }
}

function inTagEdit(index) {
    tagEditId.value = index;
    // 备份
    if (inKindFilters.value[index].text.length !== 0) {
        back = inKindFilters.value[index].text;
    }
    // 切换状态为编辑中
    editing.value = true;
}

function inTagNew() {
    const newTag = {
        text: "",
        value: "",
    }
    // 插入inkind并进入编辑模式
    inKindFilters.value.push(newTag);
    inTagEdit(inKindFilters.value.length - 1);
}

// 编辑保存
function saveInEdit(text) {
    // 首先判断是否非空
    if (text.length === 0) {
        inKindFilters.value[tagEditId.value].text = back;
        // 弹出警告
        ElNotification({
            title: '错误',
            message: '标签值不能输入空值！',
            type: 'error',
            position: 'top-right'
        })
    } else {
        inKindFilters.value[tagEditId.value].value = "收入："+text;
        tagEditId.value = null;
        editing.value = false;
    }
}

// 保存按钮
async function saveIn() {
    // 将config对应值更新
    config.value.in_tag = inKindFilters.value;
    // 关闭弹出框
    inTagDialog.value = false;
    // 保存config
    await save();
}

// 取消按钮
function cancelIn() {
    // 将inkind还原
    inKindFilters.value = configBack;
    console.log("还原发生!");
    console.log(config.value.in_tag);
    // 关闭弹出窗
    inTagDialog.value = false;
    // 编辑状态还原
    tagEditId.value = null;
    editing.value = false;
}

function outTagDelete(index) {
    outKindFilters.value.splice(index, 1);

    // 若移除的是正在编辑中的栏目，移除编辑状态
    if ((index === outKindFilters.value.length) && editing.value) {
        tagEditId.value = null;
        editing.value = false;
    }
}

function outTagEdit(index) {
    tagEditId.value = index;
    // 备份
    if (outKindFilters.value[index].text.length !== 0) {
        back = outKindFilters.value[index].text;
    }
    // 切换状态为编辑中
    editing.value = true;
}

function outTagNew() {
    const newTag = {
        text: "",
        value: "",
    }
    // 插入inkind并进入编辑模式
    outKindFilters.value.push(newTag);
    outTagEdit(outKindFilters.value.length - 1);
}

// 编辑保存
function saveOutEdit(text) {
    // 首先判断是否非空
    if (text.length === 0) {
        outKindFilters.value[tagEditId.value].text = back;
        // 弹出警告
        ElNotification({
            title: '错误',
            message: '标签值不能输入空值！',
            type: 'error',
            position: 'top-right'
        })
    } else {
        outKindFilters.value[tagEditId.value].value = "支出："+text;
        tagEditId.value = null;
        editing.value = false;
    }
}

// 保存按钮
async function saveOut() {
    // 将config对应值更新
    config.value.out_tag = outKindFilters.value;
    // 关闭弹出框
    outTagDialog.value = false;
    // 保存config
    await save();
}

// 取消按钮
function cancelOut() {
    // 将inkind还原
    outKindFilters.value = configBack;
    // 关闭弹出窗
    outTagDialog.value = false;
    // 编辑状态还原
    tagEditId.value = null;
    editing.value = false;
}

async function planBlur() {
    config.value.expect_out = expect_out.value;
    console.log("设置更改", config.value);
    await save();
}

async function save() {
    const configString = JSON.stringify(config.value);
    await invoke('save_config', {config: configString});
}
</script>

<template>
    <div class="plan-group">
        <div class="group-title">计划</div>
        <el-card class="setting-card" shadow="never">
            <div class="card-content">
                <div class="text-section">
                    <div class="setting-name">月度计划金额</div>
                    <div class="setting-des">每月的期望支出</div>
                </div>
                <div class="button-section">
                    <el-input-number v-model="expect_out" :precision="2" :controls="false" @blur="planBlur"/>
                </div>
            </div>
        </el-card>
    </div>

    <div class="tag-group">
        <div class="group-title">标签</div>
        <el-card class="setting-card" shadow="never">
            <div class="card-content">
                <div class="text-section">
                    <div class="setting-name">收入标签</div>
                    <div class="setting-des">自定收入种类标签的数量与内容</div>
                </div>
                <div class="button-section">
                    <el-button type="primary" @click="inTagBegin">编辑</el-button>
                </div>
            </div>
        </el-card>
        <el-card class="setting-card" shadow="never">
            <div class="card-content">
                <div class="text-section">
                    <div class="setting-name">支出标签</div>
                    <div class="setting-des">自定支出种类标签的数量与内容</div>
                </div>
                <div class="button-section">
                    <el-button type="primary" @click="outTagBegin">编辑</el-button>
                </div>
            </div>
        </el-card>
    </div>

    <!-- 消息弹出框 -->
     <el-dialog v-model="inTagDialog" title="收入标签" width="800" :close-on-click-modal="false" :close-on-press-escape="false" :show-close="false">
        <div class="dialog-content">
            <el-table :data="inKindFilters">
                <el-table-column prop="text" label="标签名" align="center">
                    <template #default="scope">
                        <el-input
                            v-if="tagEditId === scope.$index"
                            v-model="scope.row.text" />
                        <span v-else>{{ scope.row.text }}</span>
                    </template>
                </el-table-column>

                <el-table-column label="操作" align="center">
                    <template #default="scope">
                        <el-button v-if="tagEditId === scope.$index" type="primary" size="small" @click="saveInEdit(scope.row.text)">保存</el-button>
                        <el-button v-else type="primary" size="small" @click="inTagEdit(scope.$index)" :disabled="editing">编辑</el-button>
                        <el-button type="danger" size="small" @click="inTagDelete(scope.$index)">移除</el-button>
                    </template>
                </el-table-column>
            </el-table>
            <el-button style="margin-top: 5px;" :disabled="editing" @click="inTagNew">新增</el-button>
        </div>

        <template #footer>
            <el-button type="primary" :disabled="editing" @click="saveIn">保存</el-button>
            <el-button type="danger" @click="cancelIn">取消</el-button>
        </template>
     </el-dialog>

     <el-dialog v-model="outTagDialog" title="支出标签" width="800" :close-on-click-modal="false" :close-on-press-escape="false" :show-close="false">
        <div class="dialog-content">
            <el-table :data="outKindFilters">
                <el-table-column prop="text" label="标签名" align="center">
                    <template #default="scope">
                        <el-input
                            v-if="tagEditId === scope.$index"
                            v-model="scope.row.text" />
                        <span v-else>{{ scope.row.text }}</span>
                    </template>
                </el-table-column>

                <el-table-column label="操作" align="center">
                    <template #default="scope">
                        <el-button v-if="tagEditId === scope.$index" type="primary" size="small" @click="saveOutEdit(scope.row.text)">保存</el-button>
                        <el-button v-else type="primary" size="small" @click="outTagEdit(scope.$index)" :disabled="editing">编辑</el-button>
                        <el-button type="danger" size="small" @click="outTagDelete(scope.$index)">移除</el-button>
                    </template>
                </el-table-column>
            </el-table>
            <el-button style="margin-top: 5px;" :disabled="editing" @click="outTagNew">新增</el-button>
        </div>

        <template #footer>
            <el-button type="primary" :disabled="editing" @click="saveOut">保存</el-button>
            <el-button type="danger" @click="cancelOut">取消</el-button>
        </template>
     </el-dialog>
</template>

<style scoped>
.group-title {
    font-weight: 700;
    margin: 10px;
}

.card-content {
    display: flex;
    justify-content: space-between;
    align-items: center;
}

.text-section {
    flex: 1;
}

.setting-name {
    font-weight: bold;
    margin-bottom: 5px;
}

.setting-des {
    color: #606266;
}

.button-section {
    text-align: right;
}

.el-dialog {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
}

.dialog-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
}
</style>