<script setup>
import { invoke } from '@tauri-apps/api/core';
import { onMounted, ref } from 'vue';
import { test, attrsFilters, inKindFilters, outKindFilters, config } from "../main";
console.log("开始初始化Hello")

// 本日消费金额
const todayExpenses = ref(0);
const todayGet = ref(0);
// 相较昨日变化
const dayChange = ref(0);
const dayIncrease = ref(true);
// 本月消费金额
const thisMonthExpenses = ref(0);
const thisMonthGet = ref(0);
// 相较昨月变化
const monthChange = ref(0);
const monthIncrease = ref(true);
// 日均消费
const perExpenses = ref(0);
// 相较昨月变化
const perChange = ref(0);
const perIncrease = ref(true);
// 本月计划剩余金额
const remainingMoney = ref(0);

// 获取统计数据
async function get_data() {
    const [today, lastDay, thisMonth, lastMonth, thisMean, lastMean, dayGet, monthGet] = await invoke('get_hello_data');
    todayExpenses.value = today;
    console.log(today);
    console.log(lastDay);
    thisMonthExpenses.value = thisMonth;
    console.log(thisMonth);
    perExpenses.value = thisMean;
    console.log(thisMean);
    todayGet.value = dayGet;
    thisMonthGet.value = monthGet;

    // 日级变化
    if (today > lastDay) {
        dayIncrease.value = true;
        if (lastDay === 0) {
            dayChange.value = (today * 100).toFixed(2);
        } else {
            dayChange.value = ((today - lastDay) / lastDay * 100).toFixed(2);
        }
    } else if (today < lastDay) {
        dayIncrease.value = false;
        dayChange.value = ((lastDay - today) / lastDay * 100).toFixed(2);
    } else {
        dayChange.value = 0.00;
    }

    // 月级变化
    if (thisMonth > lastMonth) {
        monthIncrease.value = true;
        if (lastMonth === 0) {
            monthChange.value = thisMonth * 100;
        } else {
            monthChange.value = ((thisMonth - lastMonth) / lastMonth * 100).toFixed(2);
        }
    } else if (thisMonth < lastMonth) {
        monthIncrease.value = false;
        monthChange.value = ((lastMonth - thisMonth) / lastMonth * 100).toFixed(2);
    } else {
        monthChange.value = 0.00;
    }

    // 日均变化
    if (thisMean > lastMean) {
        perIncrease.value = true;
        if (lastMean === 0) {
            perChange.value = (thisMean * 100).toFixed(2);
        } else {
            perChange.value = ((thisMean - lastMean) / lastMean * 100).toFixed(2);
        }
    } else if (thisMean < lastMean) {
        perIncrease.value = false;
        perChange.value = ((lastMean - thisMean) / lastMean * 100).toFixed(2);
    } else {
        perChange.value = 0.00;
    }

    // 剩余金额
    console.log(config.value.expect_out);
    remainingMoney.value = config.value.expect_out - thisMonth;
}

// 加载设置选项
async function init_config() {
    var configString;
    // 读取设置选项
    if (test) {
        configString = JSON.stringify({
                expect_out: 0,
                kind: [
                {
                    text: "收入",
                    value: "收入"
                },
                {
                    text: "支出",
                    value: "支出"
                }
                ],
                in_tag: [
                {
                    text: "未注明",
                    value: "收入：未注明"
                },
                {
                    text: "工资",
                    value: "收入：工资"
                }
                ],
                out_tag: [
                {
                    text: "未注明",
                    value: "支出：未注明"
                },
                {
                    text: "餐费",
                    value: "支出：餐费"
                }
                ]
            });
    } else {
        configString = await invoke("get_config");
    }

    config.value = JSON.parse(configString);
    console.log("设置初始化完成:", config.value);
    attrsFilters.value = config.value.kind;
    inKindFilters.value = config.value.in_tag;
    outKindFilters.value = config.value.out_tag;
}

// 初始化钩子
onMounted(async () => {
    console.log("概览页面初始化钩子");
    await init_config();
    await get_data();
});
</script>

<template>
    <div class="hello-carousel">
        <el-carousel 
            height="200px" 
            type="card"
            interval="10000">
            <el-carousel-item>
                <span class="first">欢迎来到斐记！</span>
            </el-carousel-item>
            <el-carousel-item>
                <h3 class="small justify-center" text="2xl">欢迎提出改进意见！</h3>
            </el-carousel-item>
        </el-carousel>
    </div>

    <!-- 数据统计 -->
     <div class="statistic">
        <el-row :gutter="16">
            <el-col :span="8">
                <div class="statistic-card">
                    <el-statistic :value="todayExpenses" precision="2">
                        <template #title>
                            <div class="statistic-header">
                                本日消费
                            </div>
                        </template>
                    </el-statistic>
                    <div class="statistic-footer">
                        <div class="footer-item">
                            <span>较昨日</span>
                            <span v-if="dayChange === 0">
                                {{ dayChange }}%
                                <el-icon><Minus /></el-icon>
                            </span>
                            <span class="red" v-else-if="dayIncrease">
                                {{ dayChange }}%
                                <el-icon><CaretTop /></el-icon>
                            </span>
                            <span class="green" v-else>
                                {{ dayChange }}%
                                <el-icon><CaretBottom /></el-icon>
                            </span>
                        </div>
                    </div>
                </div>
            </el-col>

            <el-col :span="8">
                <div class="statistic-card">
                    <el-statistic :value="thisMonthExpenses" precision="2">
                        <template #title>
                            <div class="statistic-header">
                                本月消费
                            </div>
                        </template>
                    </el-statistic>
                    <div class="statistic-footer">
                        <div class="footer-item">
                            <span>较昨月</span>
                            <span v-if="monthChange === 0">
                                {{ monthChange }}%
                                <el-icon><Minus /></el-icon>
                            </span>
                            <span class="red" v-else-if="monthIncrease">
                                {{ monthChange }}%
                                <el-icon><CaretTop /></el-icon>
                            </span>
                            <span class="green" v-else>
                                {{ monthChange }}%
                                <el-icon><CaretBottom /></el-icon>
                            </span>
                        </div>
                    </div>
                </div>
            </el-col>

            <el-col :span="8">
                <div class="statistic-card">
                    <el-statistic :value="perExpenses" precision="2">
                        <template #title>
                            <div class="statistic-header">
                                本月日均消费
                            </div>
                        </template>
                    </el-statistic>
                    <div class="statistic-footer">
                        <div class="footer-item">
                            <span>较昨月</span>
                            <span v-if="perChange === 0">
                                {{ perChange }}%
                                <el-icon><Minus /></el-icon>
                            </span>
                            <span class="red" v-else-if="perIncrease">
                                {{ perChange }}%
                                <el-icon><CaretTop /></el-icon>
                            </span>
                            <span class="green" v-else>
                                {{ perChange }}%
                                <el-icon><CaretBottom /></el-icon>
                            </span>
                        </div>
                    </div>
                </div>
            </el-col>
        </el-row>

        <el-row :gutter="16" style="margin-top: 100px;">
            <el-col :span="8">
                <div class="statistic-card">
                    <el-statistic :value="todayGet - todayExpenses" precision="2">
                        <template #title>
                            <div class="statistic-header">
                                本日盈余
                            </div>
                        </template>
                    </el-statistic>
                </div>
            </el-col>

            <el-col :span="8">
                <div class="statistic-card">
                    <el-statistic :value="remainingMoney" precision="2">
                        <template #title>
                            <div class="statistic-header">
                                本月剩余计划金额
                            </div>
                        </template>
                    </el-statistic>
                </div>
            </el-col>

            <el-col :span="8">
                <div class="statistic-card">
                    <el-statistic :value="thisMonthGet - thisMonthExpenses" precision="2">
                        <template #title>
                            <div class="statistic-header">
                                本月盈余
                            </div>
                        </template>
                    </el-statistic>
                </div>
            </el-col>
        </el-row>
     </div>
</template>

<style scoped>
.el-carousel__item h3 {
    color: #475669;
    opacity: 0.75;
    line-height: 150px;
    margin: 0;
    text-align: center;
}

.el-carousel__item:nth-child(2n) {
    background-color: #99a9bf;
}

.el-carousel__item:nth-child(2n+1) {
    background-color: #d3dce6;
}

.el-carousel__item {
    display: flex;
    justify-content: center;
    align-content: center;
}

.first {
    font-size: 32px;
    display: flex;
    justify-content: center;
    align-items: center;
}

.el-col {
    text-align: center;
}

.statistic {
    margin-top: 32px;
}

.el-statistic {
    --el-statistic-content-font-size: 40px;
}

.statistic-card {
    height: 100%;
    padding: 20px;
    border-radius: 4px;
    background-color: var(--el-bg-color-overlay);
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
}

.statistic-header {
    font-size: 16px;
    display: inline-flex; align-items: center
}

.statistic-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    flex-wrap: wrap;
    font-size: 16px;
    color: var(--el-text-color-regular);
    margin-top: 16px;
}

.statistic-footer .footer-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
}

.statistic-footer .footer-item span:last-child {
  display: inline-flex;
  align-items: center;
  margin-left: 4px;
}

.green {
  color: var(--el-color-success);
}

.red {
  color: var(--el-color-error);
}
</style>