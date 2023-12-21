<script lang="ts" setup>
import * as echarts from "echarts/core"
import { computed, inject, onMounted, ref, Ref, watch } from "vue"
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
import { HeatmapChart } from "echarts/charts"
import {
  GridComponent,
  TooltipComponent,
  VisualMapComponent,
} from "echarts/components"
import Data = wakaTimeWebapp.Data
import dayjs from "dayjs"
import { FormatSecondsHtml } from "@/utils/time.ts"
import DayComponent from "@/components/report/DayComponent.vue"

const data = inject<Ref<Data | undefined>>("data") as Ref<Data | undefined>

// prettier-ignore
const months = [
  '1月','2月','3月','4月','5月','6月','7月','8月','9月','10月','11月','12月'
]

const busiestMonth = computed(() => data.value?.time_by_month[0][0] ?? 0)
const busiestMonthTime = computed(() => data.value?.time_by_month[0][1] ?? 0)

// prettier-ignore
const weekdays = [
  '周日','周一', '周二','周三','周四','周五','周六'
]

// prettier-ignore
const weekdaysEnglish = [
  'Sun','Mon', 'Tue','Wed','Thu','Fri','Sat'
]

const busiestWeekday = computed(
  () =>
    weekdays[weekdaysEnglish.indexOf(data.value?.time_by_weekday[0][0] ?? "")],
)
const busiestWeekdayTime = data.value?.time_by_weekday[0][1] ?? 0

const dayjsBaseDay = computed(() => `${data.value?.year ?? 2023}-01-01`)

const monthChart = ref<HTMLElement | null>()
const monthChartInstance = ref<echarts.ECharts | null>(null)

const weekdayChart = ref<HTMLElement | null>()
const weekdayChartInstance = ref<echarts.ECharts | null>(null)

const monthDayChart = ref<HTMLElement | null>()
const monthDayChartInstance = ref<echarts.ECharts | null>(null)

type ECOption = echarts.ComposeOption<
  | SeriesOption
  | GridOption
  | XAXisOption
  | YAXisOption
  | VisualMapComponentOption
  | TooltipComponentOption
>

const monthChartOption = computed(
  () =>
    ({
      grid: {
        top: 20,
        bottom: 20,
        left: 60,
        right: 10,
      },
      xAxis: {
        type: "category",
        data: months,
      },
      yAxis: {
        type: "category",
        data: ["时间"],
      },
      visualMap: {
        min:
          (data.value?.time_by_month
            .map((v) => v[1])
            .reduce((p, c) => Math.min(p, c), 1000000) ?? 0) / 3600,
        max:
          (data.value?.time_by_month
            .map((v) => v[1])
            .reduce((p, c) => Math.max(p, c), 0) ?? 0) / 3600,
        top: 20,
        itemHeight: 50,
        inRange: {
          color: ["#d4eaf7", "#004f6b"],
        },
      },
      tooltip: {
        formatter: (v) => `${v.data[2].toFixed(2)} 小时`,
      },
      series: [
        {
          type: "heatmap",
          data: data.value?.time_by_month.map((v) => [
            v[0] - 1,
            0,
            v[1] / 3600,
          ]),
          label: {
            show: false,
          },
        },
      ],
    }) satisfies ECOption,
)

const weekdayChartOption = computed(
  () =>
    ({
      grid: {
        top: 20,
        bottom: 20,
        left: 60,
        right: 10,
      },
      xAxis: {
        type: "category",
        data: weekdays,
      },
      yAxis: {
        type: "category",
        data: ["时间"],
      },
      visualMap: {
        min: (data.value?.time_by_weekday[6][1] ?? 0) / (3600 * 47),
        max: (data.value?.time_by_weekday[0][1] ?? 0) / (3600 * 47),
        top: 20,
        itemHeight: 50,
        inRange: {
          color: ["#d4eaf7", "#004f6b"],
        },
      },
      tooltip: {
        formatter: (v) => `${v.data[2].toFixed(2)} 小时`,
      },
      series: [
        {
          type: "heatmap",
          data: data.value?.time_by_weekday.map((v) => [
            weekdaysEnglish.indexOf(v[0]),
            0,
            v[1] / (3600 * 47),
          ]),
          label: {
            show: false,
          },
        },
      ],
    }) satisfies ECOption,
)

const monthDayChartOption = computed(
  () =>
    ({
      grid: {
        top: 20,
        bottom: 20,
        left: 60,
        right: 10,
      },
      xAxis: {
        type: "category",
        data: Array.from({ length: 10 }, (v, k) => k + 1),
      },
      yAxis: {
        type: "category",
        data: [20, 10, 0],
      },
      visualMap: {
        min: (data.value?.time_by_month_day[29][1] ?? 0) / (3600 * 12),
        max: (data.value?.time_by_month_day[0][1] ?? 0) / (3600 * 12),
        top: 20,
        itemHeight: 80,
        inRange: {
          color: ["#d4eaf7", "#004f6b"],
        },
      },
      tooltip: {
        formatter: (v) => `${v.data[2].toFixed(2)} 小时`,
      },
      series: [
        {
          type: "heatmap",
          data: data.value?.time_by_month_day.map((v) => [
            (v[0] - 1) % 10,
            2 - Math.floor((v[0] - 1) / 10),
            v[1] / (3600 * 12),
          ]),
          label: {
            show: false,
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
    HeatmapChart,
    VisualMapComponent,
    TooltipComponent,
  ])
  monthChartInstance.value = echarts.init(monthChart.value)
  weekdayChartInstance.value = echarts.init(weekdayChart.value)
  monthDayChartInstance.value = echarts.init(monthDayChart.value)

  monthChartInstance.value.setOption(monthChartOption.value)
  weekdayChartInstance.value.setOption(weekdayChartOption.value)
  monthDayChartInstance.value.setOption(monthDayChartOption.value)
})

watch(data, () => {
  monthChartInstance.value?.setOption(monthChartOption.value)
  weekdayChartInstance.value?.setOption(weekdayChartOption.value)
  monthDayChartInstance.value?.setOption(monthDayChartOption.value)
})

onresize = () => {
  monthChartInstance.value?.resize()
  weekdayChartInstance.value?.resize()
  monthDayChartInstance.value?.resize()
}
</script>

<template>
  <div class="flex flex-wrap gap-5">
    <div
      class="card w-[40vw] max-w-[500px] h-[220px] animate__animated animate__fadeInDown"
    >
      <div class="card-title">你最忙碌的一个月</div>
      <div class="font-normal">
        <p>
          你最忙碌的一个月是 <span class="d">{{ busiestMonth }}</span> 月
        </p>
        <p>
          这个月里你一共工作了
          <span class="d">{{ (busiestMonthTime / 3600).toFixed(2) }}</span
          >小时，平均每天
          <span class="d">{{
            (busiestMonthTime / (3600 * 30)).toFixed(2)
          }}</span>
          小时
        </p>
      </div>
      <div ref="monthChart" class="h-[100px] w-full"></div>
    </div>
    <div
      class="card w-[40vw] max-w-[500px] h-[30vh] max-h-[220px] animate__animated animate__fadeInDown"
    >
      <div class="card-title">你每周最忙碌的一天</div>
      <div class="font-normal">
        <p>
          你每周 <span class="d">{{ busiestWeekday }}</span> 最忙
        </p>
        <p>
          这一天里你平均做完整周
          <span class="d"
            >{{
              (
                (busiestWeekdayTime /
                  (data?.total_time ?? busiestWeekdayTime)) *
                100
              ).toFixed(1)
            }}%</span
          >
          的工作
        </p>
        <div ref="weekdayChart" class="h-[100px] w-full"></div>
      </div>
    </div>
    <div
      class="card w-[40vw] max-w-[500px] h-[30vh] max-h-[220px] animate__animated animate__fadeInDown animate__delay-1s"
    >
      <div class="card-title">你每个月最忙碌的一天</div>
      <div class="font-normal">
        <p>
          你每个月这五天最忙：
          <span class="d">
            {{
              data?.time_by_month_day
                .slice(0, 5)
                .map((v) => v[0])
                .join(", ")
            }}
          </span>
        </p>
      </div>
      <div ref="monthDayChart" class="h-[130px] w-full"></div>
    </div>
    <div
      class="card w-[40vw] max-w-[500px] h-[30vh] max-h-[220px] animate__animated animate__fadeInDown animate__delay-1s"
    >
      <div class="card-title">你全年最忙碌的一天</div>
      <div class="font-normal">
        <p>
          你在
          <span class="d">{{
            dayjs(dayjsBaseDay)
              .dayOfYear(data?.time_by_day[0][0] ?? 1)
              .format("LL")
          }}</span>
          创下了<span
            class="d2"
            v-html="
              FormatSecondsHtml(data?.time_by_day[0][1] ?? 0, true, false)
            "
          />的记录
        </p>
      </div>
      <div class="h-[130px] w-full overflow-scroll">
        <div class="my-4 flex flex-wrap gap-3">
          <day-component
            v-for="d in data?.time_by_day"
            :key="d[0]"
            :day="dayjs(dayjsBaseDay).dayOfYear(d[0]).format('M月D日')"
            :time="d[1]"
          />
        </div>
      </div>
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
  @apply text-xl font-bold;
}
</style>

<style>
.d2 span.dig {
  @apply text-xl font-bold;
}
</style>
