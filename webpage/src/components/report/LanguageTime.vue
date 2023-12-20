<script lang="ts" setup>
import * as echarts from "echarts/core"
import { computed, inject, onMounted, ref, Ref, watch } from "vue"
import { FormatSeconds, FormatSecondsHtml } from "@/utils/time.ts"
import { SeriesOption } from "echarts"
import { GridOption, XAXisOption, YAXisOption } from "echarts/types/dist/shared"
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

const chartOption = computed(() => ({
  grid: {
    top: 10,
    bottom: 30,
    left: 100,
    right: 100
  },
  xAxis: {
    type: "value",
    axisLabel: {
      show: false
    }
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
      formatter: function(value: any) {
        return value
      }
    }
  },
  series: [{
    type: "bar",
    data: data.value?.time_by_language.map((v) => v[1]),
    itemStyle: {
      color: "#3b3c3d"
    },
    label: {
      show: true,
      position: "right",
      textBorderColor: "transparent",
      formatter: (value) => FormatSeconds(Number(value.data), false)
    }
  }]
} satisfies ECOption))

onMounted(() => {
  echarts.use([
    CanvasRenderer, LabelLayout, UniversalTransition, BarChart, GridComponent
  ])
  chartInstance.value = echarts.init(chart.value)
  chartInstance.value.setOption(chartOption.value)
})

watch(data, () => {
  chartInstance.value?.setOption(chartOption.value)
})

</script>

<template>
  <div class="text-4xl font-extrabold animate__animated animate__fadeInDown">
    <p>你最常用的语言是 <span class="text-6xl">{{ languageTime[0][0] }}</span></p>
    <p class="mt-3">它陪伴你走过了 <span
      class="d"
      v-html="FormatSecondsHtml(languageTime[0][1], true)"
    /></p>
    <p class="mt-10 text-3xl">下面是你常用的语言排行：</p>
    <div ref="chart" class="mt-10" style="height: 350px;width: 700px"></div>
  </div>
</template>

<style scoped></style>
