import 'vue-router'

declare module 'vue-router' {
  interface RouteMeta {
    toTop?: boolean
    transition?: 'slide-left' | 'slide-right'
  }
}
