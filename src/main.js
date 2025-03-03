import { createApp, ref } from "vue";
import App from "./App.vue";
import ElementPlus from 'element-plus';
import * as ElementPlusIconsVue from '@element-plus/icons-vue'
import 'element-plus/dist/index.css';
import './assets/styles/main.css'

import router from "./router";

const app = createApp(App);
for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
    app.component(key, component)
}

export const test = false;
export const asideSelected = ref('hello');

export const config = ref(null);
export const attrsFilters = ref(null);
export const inKindFilters = ref(null);
export const outKindFilters = ref(null);

export const tableData = ref([]);
// 数据读取指针，代表查询数据月份与当前日期月的相对值
export const dataLoadPtr = ref(0);
// 数据存储指针
export const dataSavePtr = ref(0);
// 驻日图表数据
export const tableStatData = ref([]);

app.use(ElementPlus);
app.use(router);
app.mount('#app');