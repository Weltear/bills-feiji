<script setup>
import { invoke } from '@tauri-apps/api/core';
import * as echarts from 'echarts';
import { nextTick, onBeforeUnmount, onMounted, ref } from 'vue';

var myChart;
var pieChart;
// 表格数据
const data = ref([]);
const pieData = ref([]);
const now = new Date();
var yearAxis = ['一月', '二月', '三月', '四月', '五月', '六月', '七月', '八月', '九月', '十月', '十一月', '十二月'];
const days = new Date(now.getFullYear(), now.getMonth() + 1, 0).getDate();
var monthAxis = Array.from({length: days}, (_, i) => i + 1);

// 选择年份与月份
const yearSelected = ref(now.getFullYear().toString());
const monthSelected = ref(`${now.getFullYear()}-${String(now.getMonth() + 1).padStart(2, '0')}`);
const mode = ref('年度统计');

// 抽屉弹出
const drawer = ref(false);

// 定义图标选项
const option = {
    tooltip: {
        trigger: 'axis',
        axisPointer: {
            type: 'shadow'
        }
    },
    legend: {},
    grid: {
        left: '3%',
        right: '4%',
        bottom: '3%',
        containLabel: true
    },
    xAxis: [
        {
            type: 'category',
            data: ['一月', '二月', '三月', '四月', '五月', '六月', '七月', '八月', '九月', '十月', '十一月', '十二月'],
        }
    ],
    yAxis: [
        {
            type: 'value'
        }
    ],
    series: data.value
};

// 饼图选项
const pieOption = {
    title: {
        text: '统计饼图',
        subtext: '支出',
        left: 'center'
    },
    tooltip: {
        trigger: 'item',
        formatter: '{a} <br/>{b} : {c} ({d}%)'
    },
    legend: {
        orient: 'vertical',
        left: 'right'
    },
    series: pieData.value
}

// 返回月份日数
function monthDays(date) {
    let days = new Date(date.getFullYear(), date.getMonth() + 1, 0).getDate();
    monthAxis = Array.from({length: days}, (_, i) => i + 1);
    console.log("该月有", monthAxis);
}

async function pieShow(param) {
    // 显示抽屉
    drawer.value = true;
    await nextTick();

    // 初始化饼图
    const pieDom = document.getElementById('pie-chart');
    pieChart = echarts.init(pieDom);

    console.log("柱状图点击参数", param);
    let stringData = JSON.stringify(data.value);
    stringData = await invoke('get_pie_data', {barSeries: stringData, index: param.dataIndex});
    pieData.value = JSON.parse(stringData);

    pieOption.series = pieData.value;
    pieChart.setOption(pieOption);
}

async function yearChange() {
    console.log("年份改变", yearSelected.value);
    // 更新数据
    const dataString = await invoke('get_stat_data', {year: Number(yearSelected.value)});
    data.value = JSON.parse(dataString);
    console.log("统计数据", data.value);

    // 更新选项
    option.series = data.value;
    option.xAxis = [
        {
            type: 'category',
            data: yearAxis
        }
    ]

    myChart.setOption(option);
}

async function monthChange() {
    console.log("月份改变", monthSelected.value);
    // 生成完整日期对象
    let date = new Date(monthSelected.value + '-01');
    // 改变日数
    monthDays(date);

    // 更新数据
    const dataString = await invoke('get_day_stat', {str: monthSelected.value});
    data.value = JSON.parse(dataString);
    console.log("统计数据", data.value);

    // 更新选项
    option.series = data.value;
    option.xAxis = [
        {
            type: 'category',
            data: monthAxis
        }
    ]

    myChart.setOption(option);
}

async function selectChange() {
    console.log("模式改变", mode.value);
    console.log("当前年度", yearSelected.value);
    console.log("当前月度", monthSelected.value);
    if (mode.value === '年度统计') {
        await yearChange();
    } else {
        await monthChange();
    }
}

onMounted(async () => {
    await nextTick();

    const chartDom = document.getElementById('month-chart');
    
    myChart= echarts.init(chartDom);

    // 读取数据
    const dataString = await invoke('get_stat_data', {year: Number(yearSelected.value)});
    data.value = JSON.parse(dataString);
    console.log("统计数据", data.value);
    option.series = data.value;

    myChart.setOption(option);
    myChart.on('click', pieShow);
    console.log("统计页面初始化完成")
});

onBeforeUnmount(() => {
    myChart.dispose();
})
</script>

<template>
    <div class="statistics-view">
        <el-container>
            <el-header style="font-size: 32px;">账单统计</el-header>

            <el-main class="stat">
                <div id="month-chart" style="height: 600px; width: 100%;">to be continued</div>

                <div style="display: flex;">
                    <el-select v-model="mode" style="width: 100px;" @change="selectChange">
                        <el-option label="年度统计" value="年度统计"/>
                        <el-option label="月度统计" value="月度统计"/>
                    </el-select>
                    
                    <el-date-picker
                        v-model="yearSelected"
                        v-if="mode === '年度统计'"
                        type="year"
                        value-format="YYYY"
                        @change="yearChange"/>
                    
                    <el-date-picker
                        v-model="monthSelected"
                        v-else
                        type="month"
                        value-format="YYYY-MM"
                        @change="monthChange"/>
                </div>
                
            </el-main>
        </el-container>
    </div>

    <!-- 饼图弹出抽屉 -->
     <el-drawer
        v-model="drawer"
        direction="btt"
        :show-close="false"
        :size="'95%'">
        <div id="pie-chart" style="height: 600px; width: 100%;">to be continued</div>
        </el-drawer>
</template>

<style scoped>
.stat {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
}
</style>