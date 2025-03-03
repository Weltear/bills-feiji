<!-- 账目展示表格，提供多种功能 -->

<script setup>
import { invoke } from "@tauri-apps/api/core";
import { onMounted, ref } from "vue";
import { test, attrsFilters, inKindFilters, outKindFilters, tableData, dataLoadPtr, dataSavePtr } from "../main";
import data1 from "../data/2025_2.json";
import data2 from "../data/2025_1.json";

const tableLoad = ref(false);
// 日期选择数据
const monthPicked = ref('');
// 编辑行索引
const editId = ref(null);
// 编辑列索引
const editKey = ref(null)
// 整行编辑状态
const rowEdit = ref(false);
// kind总属性
const kindFilters = ref(inKindFilters.value.concat(outKindFilters.value));

async function getTableData() {
    // 在获得新的表单数据前，需要保存并清空编辑状态
    if (test) {
        console.log("抓取本地");
        if (dataLoadPtr.value == 0) {
            tableData.value = data1;
        } else if (dataLoadPtr.value == -1) {
            tableData.value = data2;
        }
    } else {
        tableLoad.value = true;
        try {
            // NOTE 貌似需要改成js风格的键名，不能用原版的data_ptr
            const [jsonData, newMonth] = await invoke('get_appointed_data', {dataPtr: dataLoadPtr.value});
            const data = JSON.parse(jsonData);
            console.log(data);
            console.log(newMonth);
            // 将data按照日期降序排序
            data.sort(mySort);
            tableData.value = data;
            monthPicked.value = newMonth;
            // 在新表格渲染完成后，将数据保存指针与数据读取指针置同
            dataSavePtr.value = dataLoadPtr.value;
        } catch(error) {
            console.error('Error fetching: ', error);
        }
        tableLoad.value = false;
    }
}

// 数据指针操作函数
async function dataPointerAdd() {
    await saveEdit();
    dataLoadPtr.value = dataLoadPtr.value + 1;
    console.log("数据指针增加，当前偏离量为", dataLoadPtr.value);
    await getTableData(dataLoadPtr.value);
}

async function dataPointerSubtract() {
    await saveEdit();
    dataLoadPtr.value = dataLoadPtr.value - 1;
    console.log("数据指针减少，当前偏离量为", dataLoadPtr.value);
    await getTableData(dataLoadPtr.value);
}

// 月份选择变动函数
async function monthPickedChanged() {
    await saveEdit();
    console.log("月份变更：", monthPicked.value);
    // 若月份变动为空，则返回当月
    if (monthPicked.value == null) {
        // 指针置零，重读数据
        dataLoadPtr.value = 0;
        console.log("指针置零，回归初始状态");
    } else {
        // 更改指针
        dataLoadPtr.value = getMonthDiff();
        console.log("月份选择导致指针变动，当前偏离量", dataLoadPtr.value);
    }
    await getTableData();
}

// 两月差值
function getMonthDiff() {
    const selectDate = new Date(monthPicked.value);
    const currentDate = new Date();

    const startYear = currentDate.getFullYear();
    const startMonth = currentDate.getMonth();
    const endYear = selectDate.getFullYear();
    const endMonth = selectDate.getMonth();

    return (endYear - startYear) * 12 + (endMonth - startMonth)
}

// 表格操作栏函数
async function removeRow(index) {
    tableData.value.splice(index, 1);
    await saveEdit();
}

function editRow(index) {
    console.log(tableData.value[index]);
    editId.value = index;
    rowEdit.value = true;
}

function addRow() {
    // 获取当前日期
    const now = new Date();
    const year = now.getFullYear();
    const month = String(now.getMonth() + 1).padStart(2, '0');
    const day = String(now.getDate()).padStart(2, '0');
    // 在表格开头插入新值
    tableData.value.unshift({
        id: tableData.value.length,
        value: 0,
        attribute: "支出",
        kind: outKindFilters.value[0].text,
        date: `${year}-${month}-${day}`,
        note: ""
    });
    // 设置为当前整行编辑状态
    editRow(0);
}

// 变化保存函数，在完成编辑后在后端更新
async function saveEdit() {
    // 首先，将编辑状态相关变量置null关闭编辑
    editId.value = null;
    editKey.value = null;
    rowEdit.value = false; 

    // 调用后端接口更新数据，数组逆转还原交给后端
    console.log("传输后端：", tableData.value)
    await invoke('save_appointed_data', {data: JSON.stringify(tableData.value), dataPtr: dataLoadPtr.value});

    // 数据保存完成后，进行排序
    tableChanged();
}

function filterTag(value, row, column) {
    // NOTE 这里的property代表了column中定义的prop
    const property = column['property'];
    return row[property] === value;
}

// 属性下拉菜单变更,command为被选中的value
function attrDropdownCommand(command) {
    console.log("属性被选择：", command)
    tableData.value[editId.value].attribute = command;
    valueInputChanged(tableData.value[editId.value].value);
    // 同时，调整同行的kind
    if (command === "收入") {
        kindDropdownCommand(inKindFilters.value[0].text);
    } else {
        kindDropdownCommand(outKindFilters.value[0].text);
    }
    // 若不处于整行编辑模式，完成选择后视为失焦
    if (!rowEdit.value) {
        cellBlur();
    }
}

// 类型下拉菜单变更，command为选中的value
function kindDropdownCommand(command) {
    console.log("类型被选择", command);
    tableData.value[editId.value].kind = command;
    // 若不处于整行编辑模式，完成选择后视为失焦
    if (!rowEdit.value) {
        cellBlur();
    }
}

// 账目值输入变化监听函数，根据种类为收入或支出调整正负，同时联系到属性变化函数
function valueInputChanged(value) {
    if (tableData.value[editId.value].attribute == '收入') {
        if (value < 0) {
            tableData.value[editId.value].value = -value;
        }
    } else {
        if (value > 0) {
            tableData.value[editId.value].value = -value;
        }
    }
}

// 单元格双击触发函数，作为单个编辑
function cellEdit(row, column, cell, event) {
    console.log("被双击！")
    console.log(row, column, cell, event);
    // 获取被双击的列名
    const property = column.property;
    // 获取被双击的行数
    const index = tableData.value.indexOf(row);
    // 若未处于整行编辑状态下，进行更新
    if (!rowEdit.value) {
        editKey.value = property;
        editId.value = index;
    }
    console.log("当前列索引", editKey.value);
    console.log("当前行索引", index);
}

// 单元格失焦函数
async function cellBlur() {
    // 首先，若在整行编辑模式下，则失焦保存不启用
    if (!rowEdit.value) {
        console.log("单元格失焦！")
        await saveEdit();
    }
}

// 时间选择限制函数，只允许选择当前页面月份的时间
function dateLimited(date) {
    const now = new Date();
    return !(now.getMonth() === date.getMonth());
}

// 表格排序函数，按照日期降序排序
// 需要在渲染完成、数据更改后调用
function tableChanged() {
    tableData.value.sort(mySort);
}

// 首先根据日期，其次根据序号排序
function mySort(a, b) {
    if (a['date'] !== b['date']) return a['date'] < b['date'] ? 1: -1
    else if (a['id'] !== b['id']) return a['id'] < b['id'] ? 1: -1
}

// 初始化钩子
onMounted(async () => {
    console.log("查询页面初始化钩子");
    await getTableData();
});
</script>

<template>
    <div class="AccsTable">
        <div class="buttons">
            <el-button type="primary" @click="addRow" class="addButton" style="margin-bottom: 5px;">新增</el-button>
        </div>

        <el-table 
            style="height: 65vh;"
            :data="tableData"
            empty-text="暂无账目"
            @cell-dblclick="cellEdit"
            v-loading="tableLoad"
            element-loading-text="读取中...">

                    <el-table-column prop="date" label="日期" width="180" align="center">
                        <!-- 作用域插槽自定义单元格 -->
                         <template #default="scope">
                            <el-date-picker
                                v-model="scope.row.date"
                                v-if="(editId === scope.$index) && ((editKey === 'date') || rowEdit)"
                                :disabled-date="dateLimited"
                                @blur="cellBlur"
                                @change="cellBlur"
                                value-format="YYYY-MM-DD" />
                            <span v-else>{{ scope.row.date }}</span>
                         </template>
                    </el-table-column>
                    

                    <el-table-column prop="attribute" label="属性" width="120" align="center"
                        :filters="attrsFilters"
                        :filter-method="filterTag">
                        <template #default="scope">
                            <el-select
                                v-if="(editId === scope.$index) && ((editKey === 'attribute') || rowEdit)"
                                v-model="scope.row.attribute"
                                filterable
                                @change="attrDropdownCommand"
                                @blur="cellBlur"
                                style="width: 80px;">
                                <el-option
                                    v-for="item in attrsFilters"
                                    :key="item.value"
                                    :label="item.text"
                                    :value="item.value"/>
                            </el-select>
                            <span v-else>{{ scope.row.attribute }}</span>
                        </template>
                    </el-table-column>


                    <el-table-column prop="kind" label="种类" width="150" align="center"
                        :filters="kindFilters"
                        :filter-method="filterTag">
                        <template #default="scope">
                            <el-select
                                v-if="(editId === scope.$index) && ((editKey === 'kind') || rowEdit)"
                                v-model="scope.row.kind"
                                allow-create
                                default-first-option
                                filterable
                                @change="kindDropdownCommand"
                                @blur="cellBlur"
                                style="width: 120px;">
                                <div v-if="scope.row.attribute === '收入'">
                                    <el-option
                                        v-for="item in inKindFilters"
                                        :key="item.value"
                                        :label="item.text"
                                        :value="item.value"/>
                                </div>
                                <div v-if="scope.row.attribute === '支出'">
                                    <el-option
                                        v-for="item in outKindFilters"
                                        :key="item.value"
                                        :label="item.text"
                                        :value="item.value"/>
                                </div>
                            </el-select>
                            <span v-else>{{ scope.row.kind }}</span>
                        </template>
                    </el-table-column>

                    <el-table-column prop="value" label="数值" width="180" align="center">
                        <template #default="scope">
                            <el-input-number 
                                v-model="scope.row.value" 
                                v-if="(editId === scope.$index) && ((editKey === 'value') || rowEdit)"
                                :precision="2"
                                :controls="false"
                                @change="valueInputChanged"
                                @blur="cellBlur"
                                />
                            <span v-else>{{ scope.row.value }}</span>
                        </template>
                    </el-table-column>

                    <el-table-column prop="note" label="备注" align="center">
                        <template #default="scope">
                            <el-input
                                v-model="scope.row.note"
                                v-if="(editId === scope.$index) && ((editKey === 'note') || rowEdit)"
                                @blur="cellBlur"
                                />
                            <span v-else>{{ scope.row.note }}</span>
                        </template>
                    </el-table-column>

                    <el-table-column fixed="right" label="操作" min-width="160" align="center">
                        <template #default="scope">
                            <el-button v-if="rowEdit && (editId === scope.$index)" type="primary" size="small" @click="saveEdit">保存</el-button>
                            <!-- 为防止多重编辑错误，在某行编辑过程中，其余行编辑功能禁用 -->
                            <el-button v-else type="primary" size="small" @click="editRow(scope.$index)" :disabled="rowEdit">编辑</el-button>
                            <el-button type="primary" size="small" @click="removeRow(scope.$index)">移除</el-button>
                        </template>
                    </el-table-column>
         </el-table>
    </div>

    <div class="PageButtons">
        <el-button
            class="LastMonthButton"
            @click="dataPointerSubtract">
            上一月
        </el-button>

        <el-date-picker
            v-model="monthPicked"
            type="month"
            placeholder="选择月份"
            @change="monthPickedChanged"/>

        <el-button
            class="NextMonthButton"
            @click="dataPointerAdd">
            下一月
        </el-button>
    </div>
</template>

<style scoped>
.PageButtons {
    /* 设为flex容器 */
    display: flex;
    /* 水平居中对齐子元素 */
    justify-content: center;
    /* 垂直居中对齐子元素 */
    align-items: center;
    /* 添加上下间距 */
    margin: 10px 0px;
}

.LastMonthButton {
    margin-right: 16px;
}

.NextMonthButton {
    margin-left: 16px;
}

.addButton {
    margin-bottom: 16px;
}
</style>