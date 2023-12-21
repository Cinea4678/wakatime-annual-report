import { createApp } from "vue"
import "./style.css"
import App from "./App.vue"
import router from "./utils/router.ts"

import "animate.css/animate.min.css"

import "dayjs/locale/zh"
import LocalizedFormat from "dayjs/plugin/localizedFormat"
import DayOfYear from "dayjs/plugin/dayOfYear"
import dayjs from "dayjs"
dayjs.extend(LocalizedFormat)
dayjs.extend(DayOfYear)
dayjs.locale("zh")

console.log(dayjs().format("LLL"))

createApp(App).use(router).mount("#app")
