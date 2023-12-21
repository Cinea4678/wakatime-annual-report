<script lang="ts" setup>
import { computed, inject, Ref } from "vue"
import { FormatSecondsHtml } from "@/utils/time.ts"
import Data = wakaTimeWebapp.Data
import ProjectComponent from "@/components/report/ProjectComponent.vue"

const data = inject<Ref<Data | undefined>>("data") as Ref<Data | undefined>
const projectTime = computed(
  () => data.value?.time_by_project.slice(0, 10) ?? [],
)
</script>

<template>
  <div class="text-2xl lg:text-4xl font-extrabold">
    <div class="animate__animated animate__fadeInDown">
      <p class="">
        你在 {{ data?.year }} 年内花费时间最多的项目是
        <span class="text-6xl">{{ projectTime[0][0] }}</span>
      </p>
      <p class="mt-3">
        你为它花了
        <span class="d" v-html="FormatSecondsHtml(projectTime[0][1], true)" />
      </p>
    </div>
    <div class="animate__animated animate__fadeInDown animate__delay-1s">
      <p class="mt-10 text-xl lg:text-3xl">你今年的十大项目：</p>
      <div class="mt-5 flex flex-wrap gap-3">
        <project-component
          v-for="(p, i) in projectTime"
          :key="i"
          :name="p[0]"
          :time="p[1]"
        />
      </div>
    </div>
  </div>
</template>

<style scoped></style>
