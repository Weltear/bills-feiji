<script setup>
import { invoke } from '@tauri-apps/api/core';
import * as echarts from 'echarts';
import { onBeforeUnmount, onMounted, ref, watch } from 'vue';
import { dataLoadPtr, tableStatData } from '../main';

var myChart;

// 获取本月天数
const now = new Date();
const days = new Date(now.getFullYear(), now.getMonth() + 1, 0).getDate();
const dayArray = Array.from({length: days}, (_, i) => i + 1);
console.log("dayArray is ", dayArray);

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
            data: dayArray,
        }
    ],
    yAxis: [
        {
            type: 'value'
        }
    ],
    series: tableStatData.value
};

onMounted(async () => {
    const chartDom = document.getElementById('day-chart');
    myChart= echarts.init(chartDom);

    // 读取数据
    const dataString = await invoke('get_day_stat', {dataPtr: dataLoadPtr.value});
    tableStatData.value = JSON.parse(dataString);
    console.log("统计数据", tableStatData.value);
    option.series = tableStatData.value;

    myChart.setOption(option);
    console.log("统计页面初始化完成")
});

onBeforeUnmount(() => {
    myChart.dispose();
});

// 监听
watch(tableStatData, (_) => {
    option.series = tableStatData.value;
    myChart.setOption(option);
})
</script>

<template>
    <div class="table-statistics">
        <el-scrollbar>
            <div id="day-chart" style="height: 75vh; width: 1300px;">to be continued</div>
        </el-scrollbar>
    </div>
</template>