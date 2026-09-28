import { createRouter, createWebHashHistory } from 'vue-router';

import Layout from '@/layout/Layout.vue';
import { useUserStore } from '@/stores/user';

const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    {
      path: '/',
      redirect: '/home',
      component: Layout,
      children: [
        {
          path: '/home',
          name: 'Home',
          component: () => import('@/views/Home/Home.vue'),
        },
        {
          path: '/setting',
          name: 'Setting',
          component: () => import('@/views/Setting/Setting.vue'),
        },
        {
          path: '/local-list-table',
          name: 'LocalListTable',
          component: () => import('@/views/LocalList/Table.vue'),
        },
        {
          path: '/user-playlist-table/:gid',
          name: 'UserPlaylistTable',
          component: () => import('@/views/UserPlaylist/Table.vue'),
        },
        {
          path: '/search',
          name: 'Search',
          component: () => import('@/views/Search/Search.vue'),
        },
        {
          path: '/artist-list-more',
          name: 'ArtistListMore',
          component: () => import('@/views/ArtistList/More.vue'),
        },
        {
          path: '/artist-list-table/:id',
          name: 'ArtistListTable',
          component: () => import('@/views/ArtistList/Table.vue'),
        },
        {
          path: '/rank-top-more',
          name: 'RankTopMore',
          component: () => import('@/views/RankTop/More.vue'),
        },
        {
          path: '/top-playlist-more',
          name: 'TopPlaylistMore',
          component: () => import('@/views/TopPlaylist/More.vue'),
        },
        {
          path: '/top-playlist-table/:gid',
          name: 'TopPlaylistTable',
          component: () => import('@/views/TopPlaylist/Table.vue'),
        },
      ],
    },
    {
      path: '/desktop-lyric',
      name: 'DesktopLyric',
      component: () => import('@/windows/desktop-lyric/DesktopLyric.vue'),
    },
    {
      path: '/mini-player',
      name: 'MiniPlayer',
      component: () => import('@/windows/mini-player/MiniPlayer.vue'),
    },
  ],
});

let lastPos = window.history.state?.position ?? 0;

// 判断进入歌单路由时是否是登录状态
router.beforeEach((to) => {
  if (to.name === 'UserPlaylistTable') {
    const userStore = useUserStore();
    if (!userStore.userinfo) return { path: '/' };
  }
});

// 判断路由前进还是后退,使用不同的过渡效果
router.afterEach((to) => {
  const pos = window.history.state?.position ?? 0;
  to.meta.transition = pos >= lastPos ? 'slide-left' : 'slide-right';
  lastPos = pos;
});

export default router;
