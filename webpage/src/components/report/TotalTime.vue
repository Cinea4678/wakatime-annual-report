<script lang="ts" setup>
import { computed, inject, Ref } from "vue"
import { FormatSecondsHtml } from "@/utils/time.ts"
import Data = wakaTimeWebapp.Data

const data = inject<Ref<Data | undefined>>("data") as Ref<Data | undefined>

const describeWord = computed(() => {
  const time = data.value?.total_time ?? 0

  // 下面的描述语可能不太得当，作者主要还是在以学生的身份揣摩怎么写比较好，如有不周欢迎在issue中讨论
  if (time > 1450 * 3600) {
    return "你是当之无愧的超级码王"
  } else if (time > 730 * 3600) {
    return "你对代码的爱好远超常人"
  } else {
    return ""
  }
})
</script>

<template>
  <div class="pt-[5vh] text-4xl font-extrabold animate__animated animate__fadeInDown">
    <p>在过去的{{ data?.year }}年里，你在代码上花了</p>
    <p
      class="mt-3 d"
      v-html="FormatSecondsHtml(data?.total_time ?? 0, true)"
    ></p>
    <p class="mt-10">代码占据了你 {{ data?.year }} 的 {{ Math.ceil((data?.total_time ?? 0) / (365 * 36 * 24)) }}% </p>
    <p class="mt-5">{{ describeWord }}</p>
  </div>
</template>

<style>
.d span.dig {
  @apply text-6xl italic;
}
</style>
