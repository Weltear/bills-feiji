<script setup>
import { Coin, House, Money, PieChart, Setting } from '@element-plus/icons-vue';
import { watch } from 'vue';
import { useRouter } from 'vue-router';
import { asideSelected } from '../main';


// 加载设置选项

// 侧边栏选项
const options = [
    {label: '概览', value: "hello", icon: House},
    {label: '日常开销', value: "inquire", icon: Coin},
    {label: '大额开支', value: "big", icon: Money},
    {label: '账单统计', value: 'statistics', icon: PieChart},
    {label: '设置', value: 'setting', icon: Setting}
]

// 侧边栏更改响应函数，跳转路由
const router = useRouter();
// 监听
watch(asideSelected, (newValue) => {
    console.log("侧边栏切换为：", newValue);
    router.push({name: newValue});
})
</script>

<!-- 默认布局，含有侧边导航栏 -->

<template>
    <div class="default-layout" style="background-color: #f6f6f6;">
        <!-- NOTE container在包含竖直元素时，垂直排列，否则水平排列 -->
         <el-container>
            <el-aside width="180px">
                <el-scrollbar style="background-color: white;">
                    <el-segmented
                        v-model="asideSelected"
                        size="large"
                        block
                        :options="options.slice(0, -1)"
                        direction="vertical"
                        @change="routerChange"
                        style="background-color: white;"
                        >
                        <template #default="scope">
                            <div class="flex flex-col items-center gap-2 p-2">
                                <el-icon size="20">
                                    <component :is="scope.item.icon" />
                                </el-icon>
                                <div>{{ scope.item.label }}</div>
                            </div>
                        </template>
                    </el-segmented>

                    <!-- 底部设置区域 -->
                    <div class="aside-footer">
                        <el-segmented
                            v-model="asideSelected"
                            size="large"
                            block
                            :options="options.slice(-1)"
                            direction="vertical"
                            @change="routerChange"
                            style="background-color: white;"
                            >
                            <template #default="scope">
                                <div class="flex flex-col items-center gap-2 p-2">
                                    <el-icon size="20">
                                        <component :is="scope.item.icon" />
                                    </el-icon>
                                    <div>{{ scope.item.label }}</div>
                                </div>
                            </template>
                        </el-segmented>
                    </div>
                </el-scrollbar>
            </el-aside>
            <el-main style="height: 100vh;">
                <el-scrollbar>
                    <router-view />
                </el-scrollbar>
            </el-main>
         </el-container>
    </div>
</template>

<style scoped>
/* 去掉router-link的下划线 */

a{
    text-decoration: none;
}

.aside-menu-text {
    font-size: 16px;
    margin: 0 auto;
}

.el-menu-item {
    margin: 0 auto;
}

.aside-footer {
    position: absolute;
    bottom: 0;
    left: 0;
    width: 100%;
    border-top: 2px solid #eaeaea;
}

.el-main {
    margin-bottom: 0px;
    padding-bottom: 0px;
}
</style>