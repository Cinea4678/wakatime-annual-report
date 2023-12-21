<script lang="ts" setup>
import * as echarts from "echarts/core"
import { computed, inject, onMounted, ref, Ref, watch } from "vue"
import { FormatSeconds, FormatSecondsHtml } from "@/utils/time.ts"
import { SeriesOption } from "echarts"
import type {
  GridOption,
  XAXisOption,
  YAXisOption,
} from "echarts/types/dist/shared"
import { CanvasRenderer } from "echarts/renderers"
import { LabelLayout, UniversalTransition } from "echarts/features"
import { BarChart } from "echarts/charts"
import { GridComponent } from "echarts/components"
import Data = wakaTimeWebapp.Data

const data = inject<Ref<Data | undefined>>("data") as Ref<Data | undefined>
const languageTime = computed(() => data.value?.time_by_language ?? [])

const chart = ref<HTMLElement | null>()
const chartInstance = ref<echarts.ECharts | null>(null)

type ECOption = echarts.ComposeOption<
  SeriesOption | GridOption | XAXisOption | YAXisOption
>

const chartOption = computed(
  () =>
    ({
      grid: {
        top: 20,
        bottom: 30,
        left: 100,
        right: 10,
      },
      xAxis: {
        type: "value",
        axisLabel: {
          show: false,
        },
      },
      yAxis: {
        type: "category",
        data: data.value?.time_by_language.map((v) => v[0]),
        inverse: true,
        max: 10,
        axisLabel: {
          show: true,
          fontSize: 14,
          color: "#3b3c3d",
          formatter: function (value: any) {
            return value
          },
        },
      },
      series: [
        {
          type: "bar",
          data: data.value?.time_by_language.map((v) => v[1]),
          itemStyle: {
            color: "#3b3c3d",
          },
          label: {
            show: true,
            position: "right",
            textBorderColor: "transparent",
            formatter: (value) => FormatSeconds(Number(value.data), false),
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
    BarChart,
    GridComponent,
  ])
  chartInstance.value = echarts.init(chart.value)
  chartInstance.value.setOption(chartOption.value)
})

watch(data, () => {
  chartInstance.value?.setOption(chartOption.value)
})

onresize = () => {
  chartInstance.value?.resize()
}
</script>

<template>
  <div class="text-2xl lg:text-4xl font-extrabold">
    <div class="animate__animated animate__fadeInDown">
      <p>
        你最常用的语言是 <span class="text-6xl">{{ languageTime[0][0] }}</span>
      </p>
      <p class="mt-3">
        它陪伴你走过了
        <span class="d" v-html="FormatSecondsHtml(languageTime[0][1], true)" />
      </p>
    </div>
  </div>
  <div
    class="mt-10 flex animate__delay-1s animate__animated animate__fadeInDown"
  >
    <div class="card w-[60vw] max-w-[800px] h-[360px]">
      <p class="card-title">你最常用的语言排行</p>
      <div ref="chart" class="w-full h-[320px]"></div>
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
</style>
