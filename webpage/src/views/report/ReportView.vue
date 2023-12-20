<script lang="ts" setup>
import { computed, defineAsyncComponent, ref } from "vue"
import { useWindowSize } from "@vueuse/core"

const { width } = useWindowSize()
const lg = computed(() => width.value >= 1024)

const page = ref(0)
const pages = [
  {
    name: "总时间",
    component: defineAsyncComponent(
      () => import("@/components/report/TotalTime.vue")
    )
  },
  {
    name: "语言",
    component: defineAsyncComponent(
      () => import("@/components/report/LanguageTime.vue")
    )
  }
]
</script>

<template>
  <div class="relative top-0 bottom-0 left-0 right-0 px-[3vw]">
    <div class="absolute top-[7vh]">
      <div class="font-light text-2xl">WakaTime 年度报告</div>
      <div class="mt-3">
        Page {{ page + 1 }} / {{ pages.length }} - {{ pages[page].name }}
      </div>
    </div>
    <div class="absolute top-[90vh] flex gap-3 items-center">
      <a-button v-show="page > 0" :size="lg ? 'large' : 'middle'" @click="page--">
        上一页
      </a-button>
      <a-button v-if="page < pages.length - 1" :size="lg ? 'large' : 'middle'" @click="page++">
        下一页
      </a-button>
      <div v-else class="font-light">已是最后一页</div>
    </div>
    <div class="relative top-[20vh]">
      <component :is="pages[page].component" />
    </div>
  </div>
</template>

<style scoped></style>
