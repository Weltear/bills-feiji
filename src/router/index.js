import { createRouter } from 'vue-router';
import { createWebHashHistory } from 'vue-router';
import DefaultLayout from '../layouts/DefaultLayout.vue';
import Hello from '../views/Hello.vue';
import Sample from '../views/Sample.vue';
import Inquire from '../views/Inquire.vue';
import Big from '../views/Big.vue';
import Statistics from '../views/Statistics.vue';
import Setting from '../views/Setting.vue';

const router = createRouter({
    history: createWebHashHistory(),
    routes: [
        {
            path: "/",
            redirect: "/default",
        },
        {
            path: "/default",
            name: "default-introduce",
            redirect: "/default/hello",
            component: DefaultLayout,
            children: [
                {
                    path: "hello",
                    name: "hello",
                    component: Hello,
                },
                {
                    path: "inquire",
                    name: "inquire",
                    component: Inquire,
                },
                {
                    // NOTE 嵌套路由不需要/表示相对关系
                    path:"sample",
                    name: "sample",
                    component: Sample,
                },
                {
                    path: "big",
                    name: "big",
                    component: Big,
                },
                {
                    path: 'statistics',
                    name: 'statistics',
                    component: Statistics,
                },
                {
                    path: 'setting',
                    name: 'setting',
                    component: Setting,
                }
            ]
        }
    ]
})

export default router;