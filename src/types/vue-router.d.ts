import type { RouteRecordInfo } from 'vue-router';

// 用于定义parmas类型
export interface RouteNamedMap {
  Home: RouteRecordInfo<'Home', '/home', {}, {}, never>;
  Setting: RouteRecordInfo<'Setting', '/setting', {}, {}, never>;
  LocalListTable: RouteRecordInfo<'LocalListTable', '/local-list-table', {}, {}, never>;
  UserPlaylistTable: RouteRecordInfo<
    'UserPlaylistTable',
    '/user-playlist-table/:gid',
    { gid: string },
    { gid: string },
    never
  >;
  Search: RouteRecordInfo<'Search', '/search', {}, {}, never>;
  ArtistListMore: RouteRecordInfo<'ArtistListMore', '/artist-list-more', {}, {}, never>;
  ArtistListTable: RouteRecordInfo<
    'ArtistListTable',
    '/artist-list-table/:id',
    { id: string },
    { id: string },
    never
  >;
  RankTopMore: RouteRecordInfo<'RankTopMore', '/rank-top-more', {}, {}, never>;
  TopPlaylistMore: RouteRecordInfo<'TopPlaylistMore', '/top-playlist-more', {}, {}, never>;
  TopPlaylistTable: RouteRecordInfo<
    'TopPlaylistTable',
    '/top-playlist-table/:gid',
    { gid: string },
    { gid: string },
    never
  >;
}

declare module 'vue-router' {
  interface RouteMeta {
    toTop?: boolean;
    transition?: 'slide-left' | 'slide-right';
  }

  interface TypesConfig {
    RouteNamedMap: RouteNamedMap;
  }
}
