<script lang="ts" setup>
import { useWindowSize } from "@vueuse/core";
import { computed } from "vue";
import { useRouter } from "vue-router";

const router = useRouter();

const { width } = useWindowSize();
const lg = computed(() => width.value >= 1024);

const start = () => {
  router.push("/report");
};
</script>

<template>
  <div class="relative top-0 bottom-0 left-0 right-0">
    <div class="absolute top-[36vh] left-[5vw]">
      <div class="text-3xl md:text-5xl lg:text-6xl xl:text-7xl font-extrabold">
        查看你的 WakaTime 年度报告
      </div>
      <div class="mt-7 lg:mt-10 flex gap-3 items-center">
        <a-button :size="lg ? 'large' : 'middle'" @click="start">
          立即开始
        </a-button>
      </div>
      <div class="mt-10 font-light text-gray-400">
        * 本项目是独立的社区项目，与WakaTime官方没有关系
      </div>
    </div>
  </div>
</template>

<style scoped></style>
