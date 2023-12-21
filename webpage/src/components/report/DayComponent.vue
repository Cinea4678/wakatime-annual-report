<script lang="ts" setup>
import { FormatSeconds } from "@/utils/time.ts"

defineProps<{
  day: string
  time: number
}>()
</script>

<template>
  <div
    class="py-1 px-3 text-center font-bold min-w-[150px] bg-white/20 border-white/20 border backdrop-blur rounded-full"
  >
    <div class="text-[1.1em]">{{ day }}</div>
    <div class="text-sm font-normal">
      {{ FormatSeconds(time, false, false) }}
    </div>
  </div>
</template>

<style scoped></style>
