<script lang="ts" setup>
import Banner from './Banner.vue'
import ArtistList from '@/views/ArtistList/Card.vue'
import RankTop from '@/views/RankTop/Card.vue'
import TopCard from '@/views/TopCard/Card.vue'
import TopPlaylist from '@/views/TopPlaylist/Card.vue'
</script>

<template>
  <div class="main space-y-8">
    <Banner />

    <TopCard :card-id="1" />
    <RankTop />
    <TopCard :card-id="3" />
    <ArtistList />
    <TopCard :card-id="4" />
    <TopPlaylist />
    <TopCard :card-id="2" />
  </div>
</template>
