<script lang="ts" setup>
import { onMounted, provide, ref } from "vue";
import axios from "axios";
import { useRouter } from "vue-router";
import Data = wakaTimeWebapp.Data;

const router = useRouter();

const loading = ref(false);
const data = ref<Data | undefined>(undefined);
provide("data", data);

onMounted(async () => {
  loading.value = true;

  const handleError = () => {
    router.push("/error");
  };

  try {
    let res = await axios.get("/output.json");
    if (res.data.year != undefined) {
      data.value = res.data as Data;
    } else {
      handleError();
    }
  } catch (e) {
    console.log(e);
    handleError();
  } finally {
    loading.value = false;
  }
});
</script>

<template>
  <a-spin :spinning="loading">
    <div class="w-full">
      <div class="app-container">
        <router-view />
      </div>
    </div>
  </a-spin>
</template>

<style scoped>
.app-container {
  min-height: 100vh;
  font-family: "Noto Sans SC", sans-serif;
  font-weight: 300;
  max-width: 1536px;
  margin-left: auto;
  margin-right: auto;
}
</style>

<style>
body {
  background-size: cover !important;
  background: url("/bg.png") no-repeat center center fixed;
}
</style>
