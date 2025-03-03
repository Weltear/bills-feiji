<script setup>
import { invoke } from '@tauri-apps/api/core';
import { onBeforeUnmount, onMounted, ref } from 'vue';
import { onBeforeRouteLeave } from 'vue-router';

// 大额账目数组
const bigAccs = ref([]);
// 对应平均价值数组
const averageCosts = ref([]);
// 对应间隔天数
const duration = ref([]);
const isEmpty = ref(bigAccs.value.length === 0);
const dialogFormVisible = ref(false);
const columnCount = ref(3);
const newData = ref(null);
// 表单输入宽度
const formLabelWidth = '100px';
// 当前日期
const now = new Date;
// 编辑索引
const editIndex = ref(null);
const isEdit = ref(false)

// 获取账单数据
async function getData() {
    const jsonData = await invoke('big_acc_data_load');
    const data = JSON.parse(jsonData);
    // 若读取到的数据非0，则展开数据
    if (data.length !== 0) {
        bigAccs.value = data.sort(mySort);
        isEmpty.value = false;
        // 完成平均价格计算
        computeAverage();
    }
}

// 计算所有账目的平均价格
function computeAverage() {
    // 首先将平均值数组重置
    averageCosts.value = [];
    duration.value = [];

    for (let acc of bigAccs.value) {
        const start = new Date(acc.date);
        const difference = now.getTime() - start.getTime();
        // 计算间隔天数，并向上取整
        const numDays = Math.ceil(difference / (1000 * 3600 * 24));
        // 若相隔天数为0，记为1
        if (numDays === 0) {
            numDays = 1;
        }
        duration.value.push(numDays);
        averageCosts.value.push((acc.value / numDays).toFixed(2)); 
    }
}

// 元素删除
async function dataDelete(index) {
    // 数据删除
    bigAccs.value.splice(index, 1);
    // 平均值删除
    averageCosts.value.splice(index, 1);
    duration.value.slice(index, 1);
    // 若删除后，无元素则返回空界面
    if (bigAccs.value.length === 0) {
        isEmpty.value = true;
    }

    await save();
}

// 元素编辑
function dataEdit(index) {
    // 拉起表单
    formInit();
    editIndex.value = index;
    isEdit.value = true;
    // 设置预填为当前编辑项
    newData.value.id = bigAccs.value[index].id;
    newData.value.value = bigAccs.value[index].value;
    newData.value.date = bigAccs.value[index].date;
    newData.value.note = bigAccs.value[index].note;
}

// 数据保存
async function save() {
    // 将新数据提交给后台，并拉取新的数据
    const saveData = JSON.stringify(bigAccs.value);
    await invoke('big_acc_data_save', {data: saveData});
    console.log("大额开支数据已保存！");
}

// 数据排序方法，先日期后id
function mySort(a, b) {
    if (a['date'] !== b['date']) return a['date'] < b['date'] ? 1: -1
    else if (a['id'] !== b['id']) return a['id'] < b['id'] ? 1: -1
}

// 账目表单初始化
function formInit() {
    dialogFormVisible.value = true;

    const now = new Date();
    const year = now.getFullYear();
    const month = String(now.getMonth() + 1).padStart(2, '0');
    const day = String(now.getDate()).padStart(2, '0');
    
    newData.value = {
        id: bigAccs.value.length,
        value: null,
        attribute: "支出",
        kind: "",
        date: `${year}-${month}-${day}`,
        note: "",
    }
}

// 取消提交表单
function formCancel() {
    dialogFormVisible.value = false;
}

// 提交表单
async function formConfirm() {
    dialogFormVisible.value = false;
    // 强制切换为数据显示
    isEmpty.value = false;
    // 若为编辑状态，将对应索引值更改
    if (isEdit.value) {
        bigAccs.value[editIndex.value] = newData.value;
    } else {  // 否则，直接插入
        bigAccs.value.push(newData.value);
    }
    // 重新排序
    bigAccs.value.sort(mySort);
    // 重新计算平均值数组
    computeAverage();

    isEdit.value = false;

    await save();
}

getData();
// 监听路由跳转事件
// onBeforeRouteLeave(async () => {
//     await save();
// })
// // 监听页面关闭
// onMounted(() => {
//     window.addEventListener('beforeunload', save);
// })

// onBeforeUnmount(async () => {
//     window.removeEventListener('beforeunload', save);
// })
</script>

<template>
    <div class="big-view">
        <el-container>
            <el-header style="font-size: 32px;">大额开支</el-header>

            <el-main>
                <el-empty v-if="isEmpty" description="暂无数据" image-size="240">
                    <el-button type="primary" @click="formInit">立即开始</el-button>
                </el-empty>
                
                <div v-else>
                    <el-row :gutter="20">
                        <el-col :span="24 / columnCount" class="add-col">
                            <el-card class="add-card" @click="formInit">
                                <div class="button-text">
                                    <span style="font-size: 32px; margin-bottom: 5px;">+</span>
                                    <span>添加新账单</span>
                                </div>
                            </el-card>
                            
                        </el-col>
                        <el-col
                            v-for="(acc, index) in bigAccs"
                            :key="index"
                            :span="24 / columnCount"
                            class="acc-card"
                            >
                            <el-descriptions
                                :column="1"
                                border
                                label-width="120px"
                                class="show-card"
                                >
                                <el-descriptions-item label="名称" align="center">{{ acc.note }}</el-descriptions-item>
                                <el-descriptions-item label="价格" align="center">{{ acc.value }}</el-descriptions-item>
                                <el-descriptions-item label="日期" align="center">{{ acc.date }}</el-descriptions-item>
                                <el-descriptions-item label="距今天数" align="center">{{ duration[index] }}</el-descriptions-item>
                                <el-descriptions-item label="平均价格" align="center">{{ averageCosts[index] }}</el-descriptions-item>
                                <el-descriptions-item label="操作" align="center">
                                    <el-button type="primary" size="small" @click="dataEdit(index)">编辑</el-button>
                                    <el-button type="danger" size="small" @click="dataDelete(index)">删除</el-button>
                                </el-descriptions-item>
                            </el-descriptions>

                        </el-col>
                    </el-row>
                    
                </div>
            </el-main>
        </el-container>
    </div>

    <!-- 对话框 -->
    <el-dialog v-model="dialogFormVisible" title="请输入账目信息">
        <el-form :data="newData">
            <el-form-item label="开支用途" :label-width="formLabelWidth">
                <el-input v-model="newData.note"/>
            </el-form-item>

            <el-form-item label="价格" :label-width="formLabelWidth">
                <el-input-number v-model="newData.value" :precision="2" :controls="false"/>
            </el-form-item>

            <el-form-item label="日期" :label-width="formLabelWidth">
                <el-date-picker v-model="newData.date" value-format="YYYY-MM-DD"/>
            </el-form-item>
        </el-form>

        <template #footer>
            <div class="dialog-footer">
                <el-button type="primary" @click="formConfirm">提交</el-button>
                <el-button type="danger" @click="formCancel">取消</el-button>
            </div>
        </template>
    </el-dialog>
</template>

<style scoped>
.show-card {
    margin: 10px;
    border-top: 1px solid #ddd;
    border-bottom: 2px solid #bbb;
    box-shadow: 0px 0px 10px rgba(0,0,0,0.1);
}

.add-col {
    display: flex;
    justify-content: center;
    align-items: center;
}

.add-card {
    width: 120px;
    height: 120px;
    border-radius: 50%;
    display: flex;
    justify-content: center;
    align-items: center;

    cursor: pointer;
    transition: box-shadow 0.3s, transform 0.3s; /* 平滑过渡效果 */
}

.add-card:hover {
    box-shadow: 0 4px 15px rgba(0,0,0,0.2); /* 增加阴影 */
  transform: scale(1.02); /* 缩放卡片，使其看起来像是被按下的效果 */
}

.add-card:active {
    transform: scale(0.98); /* 缩小卡片，模拟按下效果 */
}

.button-text {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
}

.add-button {
    margin-bottom: 10px;
}
</style>