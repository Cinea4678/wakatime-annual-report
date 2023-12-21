<script lang="ts" setup>
import { computed, inject, onMounted, ref, Ref, watch } from "vue"
import * as echarts from "echarts/core"
import {
  SeriesOption,
  TooltipComponentOption,
  VisualMapComponentOption,
} from "echarts"
import type {
  GridOption,
  XAXisOption,
  YAXisOption,
} from "echarts/types/dist/shared"
import { CanvasRenderer } from "echarts/renderers"
import { LabelLayout, UniversalTransition } from "echarts/features"
import {
  GridComponent,
  MarkPointComponent,
  TooltipComponent,
  VisualMapComponent,
} from "echarts/components"
import { BarChart, HeatmapChart } from "echarts/charts"
import Data = wakaTimeWebapp.Data
import { LabelOption } from "echarts/types/src/util/types"
import dayjs from "dayjs"

const data = inject<Ref<Data | undefined>>("data") as Ref<Data | undefined>
const lateWorkTime = computed<Array<[number, number]> | undefined>(
  () => data.value?.late_night_time.filter((v) => v[1] > 60) ?? [],
)
const totalLateWorkTime = computed(
  () => lateWorkTime.value.reduce((p, c) => p + c[1], 0) ?? 0,
)

// prettier-ignore
const months = [
  '1月','2月','3月','4月','5月','6月','7月','8月','9月','10月','11月','12月'
]

// prettier-ignore
const weekdays = [
  '周日','周一', '周二','周三','周四','周五','周六'
]

const dayJsBase = computed(() => dayjs(`${data.value?.year}-01-01`))

// 直方图
const lateWorkTimeHistogramData = computed(
  () =>
    lateWorkTime.value
      ?.map((v) => v[1])
      .reduce(
        (p: Array<number>, c) => {
          p[Math.floor(c / 1800)]++
          return p
        },
        Array.from({ length: 11 }, () => 0),
      ),
)

// 月份分布
const lateWorkTimeMonthlyData = computed<Array<number>>(
  () =>
    lateWorkTime.value?.reduce(
      (p: Array<number>, c) => {
        p[dayJsBase.value.dayOfYear(c[0]).month()]++
        return p
      },
      Array.from({ length: 12 }, () => 0),
    ),
)

// 周内分布
const lateWorkTimeWeekdayData = computed<Array<number>>(
  () =>
    lateWorkTime.value?.reduce(
      (p: Array<number>, c) => {
        p[dayJsBase.value.dayOfYear(c[0]).weekday()]++
        return p
      },
      Array.from({ length: 7 }, () => 0),
    ),
)

const totalTimeChart = ref<HTMLElement | null>()
const totalTimeChartInstance = ref<echarts.ECharts | null>(null)

const timeHistChart = ref<HTMLElement | null>()
const timeHistChartInstance = ref<echarts.ECharts | null>(null)

const lateUpMonthChart = ref<HTMLElement | null>()
const lateUpMonthChartInstance = ref<echarts.ECharts | null>(null)

const lateUpWeekdayChart = ref<HTMLElement | null>()
const lateUpWeekdayChartInstance = ref<echarts.ECharts | null>(null)

type ECOption = echarts.ComposeOption<
  | SeriesOption
  | GridOption
  | XAXisOption
  | YAXisOption
  | VisualMapComponentOption
  | TooltipComponentOption
  | LabelOption
>

const totalTimeChartOption = computed(
  () =>
    ({
      grid: {
        top: 20,
        bottom: 20,
        left: 60,
        right: 10,
      },
      xAxis: {
        type: "value",
      },
      yAxis: {
        type: "category",
        data: ["其他时间", "熬夜时间"],
      },
      tooltip: {
        formatter: (v) => `${v.value.toFixed(2)} 小时`,
      },
      series: [
        {
          type: "bar",
          data: [
            {
              value: (data.value?.total_time - totalLateWorkTime.value) / 3600,
              itemStyle: { color: "#b6ccd8" },
              label: {
                show: true,
                formatter: "{sun|}",
                position: "insideRight",
                rich: {
                  sun: {
                    backgroundColor: {
                      image: "/sun.png",
                    },
                    width: 18,
                    height: 18,
                  },
                },
              },
            },
            {
              value: totalLateWorkTime.value / 3600,
              itemStyle: { color: "#3b3c3d" },
              label: {
                show: true,
                formatter: "{moon|}",
                position: "insideRight",
                rich: {
                  moon: {
                    backgroundColor: {
                      image: "/moon.png",
                    },
                    width: 18,
                    height: 18,
                  },
                },
              },
            },
          ],
        },
      ],
    }) satisfies ECOption,
)

const timeHistChartOption = computed(
  () =>
    ({
      grid: {
        top: 20,
        bottom: 20,
        left: 40,
        right: 10,
      },
      xAxis: {
        type: "category",
        data: [
          "<30min",
          "30min~1h",
          "1~1.5h",
          "1.5~2h",
          "2~2.5h",
          "2.5~3h",
          "3~3.5h",
          "3.5~4h",
          "4~4.5h",
          "4.5~5h",
          "5~5.5h",
        ],
      },
      yAxis: {
        type: "value",
      },
      tooltip: {
        formatter: (v) => `${v.value} 次`,
      },
      series: [
        {
          type: "bar",
          data: lateWorkTimeHistogramData.value,
          itemStyle: {
            color: "#00668c",
          },
        },
      ],
    }) satisfies ECOption,
)

const lateUpMonthChartOption = computed(
  () =>
    ({
      grid: {
        top: 20,
        bottom: 20,
        left: 40,
        right: 10,
      },
      xAxis: {
        type: "category",
        data: months,
      },
      yAxis: {
        type: "value",
      },
      tooltip: {
        formatter: (v) => `${v.value} 次`,
      },
      series: [
        {
          type: "bar",
          data: lateWorkTimeMonthlyData.value,
          itemStyle: {
            color: "#00668c",
          },
        },
      ],
    }) satisfies ECOption,
)

const lateUpWeekdayChartOption = computed(
  () =>
    ({
      grid: {
        top: 20,
        bottom: 20,
        left: 40,
        right: 10,
      },
      xAxis: {
        type: "category",
        data: weekdays,
      },
      yAxis: {
        type: "value",
      },
      tooltip: {
        formatter: (v) => `${v.value} 次`,
      },
      series: [
        {
          type: "bar",
          data: lateWorkTimeWeekdayData.value,
          itemStyle: {
            color: "#00668c",
          },
        },
      ],
    }) satisfies ECOption,
)

onMounted(() => {
  echarts.use([
    CanvasRenderer,
    LabelLayout,
    UniversalTransition,
    GridComponent,
    MarkPointComponent,
    BarChart,
    HeatmapChart,
    VisualMapComponent,
    TooltipComponent,
  ])

  totalTimeChartInstance.value = echarts.init(totalTimeChart.value)
  timeHistChartInstance.value = echarts.init(timeHistChart.value)
  lateUpMonthChartInstance.value = echarts.init(lateUpMonthChart.value)
  lateUpWeekdayChartInstance.value = echarts.init(lateUpWeekdayChart.value)

  totalTimeChartInstance.value?.setOption(totalTimeChartOption.value)
  timeHistChartInstance.value?.setOption(timeHistChartOption.value)
  lateUpMonthChartInstance.value?.setOption(lateUpMonthChartOption.value)
  lateUpWeekdayChartInstance.value?.setOption(lateUpWeekdayChartOption.value)
})

watch(data, () => {
  console.log(lateWorkTimeHistogramData.value)
  totalTimeChartInstance.value?.setOption(totalTimeChartOption.value)
  timeHistChartInstance.value?.setOption(timeHistChartOption.value)
  lateUpMonthChartInstance.value?.setOption(lateUpMonthChartOption.value)
  lateUpWeekdayChartInstance.value?.setOption(lateUpWeekdayChartOption.value)
})

onresize = () => {
  totalTimeChartInstance.value?.resize()
  timeHistChartInstance.value?.resize()
  lateUpMonthChartInstance.value?.resize()
  lateUpWeekdayChartInstance.value?.resize()
}
</script>

<template>
  <div
    class="text-2xl lg:text-4xl font-extrabold animate__animated animate__fadeInDown"
  >
    <p class="">
      {{ data?.year }} 年你共有
      <span class="text-4xl lg:text-6xl italic font-extrabold"
        >{{ lateWorkTime?.length }}
      </span>
      天在凌晨之后打开电脑
    </p>
  </div>
  <div class="mt-10 flex flex-wrap gap-5">
    <div
      class="card w-[40vw] max-w-[500px] h-[220px] animate__animated animate__fadeInDown"
    >
      <div class="card-title">
        你全年熬夜工作了
        <span class="d">{{ (totalLateWorkTime / 3600).toFixed(2) }}</span>
        小时
      </div>
      <div class="font-normal">
        <p>
          它们占你所有工作时间的
          <span class="d"
            >{{
              ((totalLateWorkTime / data?.total_time) * 100).toFixed(1)
            }}%</span
          >
        </p>
      </div>
      <div ref="totalTimeChart" class="h-[130px] w-full"></div>
    </div>
    <div
      class="card w-[40vw] max-w-[500px] h-[220px] animate__animated animate__fadeInDown"
    >
      <div class="card-title">
        你平均每次熬夜工作
        <span class="d">{{
          (totalLateWorkTime / (3600 * lateWorkTime?.length)).toFixed(2)
        }}</span>
        小时
      </div>
      <div class="font-normal">
        <p>
          你熬夜工作时间的中位数是
          <span class="d"
            >{{
              (
                lateWorkTime[Math.floor(lateWorkTime?.length / 2)][1] / 3600
              ).toFixed(1)
            }}
          </span>
          小时
        </p>
      </div>
      <div ref="timeHistChart" class="h-[130px] w-full"></div>
    </div>
    <div
      class="card w-[40vw] max-w-[500px] h-[220px] animate__animated animate__fadeInDown animate__delay-1s"
    >
      <div class="card-title">
        你在
        <span class="d">{{
          lateWorkTimeMonthlyData.indexOf(
            Math.max(...lateWorkTimeMonthlyData),
          ) + 1
        }}</span>
        月熬夜最多，达到
        <span class="d">{{ Math.max(...lateWorkTimeMonthlyData) }}</span> 次
      </div>
      <div class="font-normal mt-2">
        <p>这个月对你来说是什么特殊的回忆呢？</p>
      </div>
      <div ref="lateUpMonthChart" class="h-[130px] w-full"></div>
    </div>
    <div
      class="card w-[40vw] max-w-[500px] h-[220px] animate__animated animate__fadeInDown animate__delay-1s"
    >
      <div class="card-title">
        你每周的
        <span class="d">{{
          weekdays[
            lateWorkTimeWeekdayData.indexOf(
              Math.max(...lateWorkTimeWeekdayData),
            )
          ]
        }}</span>
        熬夜最多
      </div>
      <div class="text-gray-400 mt-2">
        <p>*注：统计星期时以凌晨时的前一天为准</p>
      </div>
      <div ref="lateUpWeekdayChart" class="h-[130px] w-full"></div>
    </div>
  </div>
</template>

<style scoped>
.card {
  @apply px-3 py-2 bg-white/20 border-white/20 border backdrop-blur rounded-2xl;
}

.card-title {
  @apply text-xl font-bold;
}

.d {
  @apply text-2xl font-bold;
}
</style>
