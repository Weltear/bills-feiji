<template>
  <keep-alive>
    <router-view v-if="$route.meta.keepAlive"/>
  </keep-alive>
  <router-view v-if="!$route.meta.keepAlive"/>
</template>

<style scoped>
@import url("./assets/styles/main.css");
</style>