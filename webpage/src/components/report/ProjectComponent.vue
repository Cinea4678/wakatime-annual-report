<script lang="ts" setup>
import { FormatSeconds } from "@/utils/time.ts"

defineProps<{
  name: string
  time: number
}>()
</script>

<template>
  <div
    class="py-3 px-5 text-center font-bold min-w-[200px] bg-white/20 border-white/20 border backdrop-blur rounded-full"
  >
    <div class="text-xl">{{ name }}</div>
    <div class="text-sm">{{ FormatSeconds(time, false, false) }}</div>
  </div>
</template>

<style scoped></style>
