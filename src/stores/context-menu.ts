import { defineStore } from 'pinia';
import { nextTick, ref } from 'vue';

interface ShowOptions {
  x: number;
  y: number;
  teleport?: string;
  options: ContextMenuOption[];
}

/** 右键菜单配置 */
export const useContextMenuStore = defineStore('context-menu', () => {
  const visible = ref(false);
  const teleport = ref('body'); // 菜单放置的节点
  const position = ref({ x: 0, y: 0 });
  const options = ref<ContextMenuOption[]>([]);

  const show = (showOptions: ShowOptions) => {
    visible.value = false;
    teleport.value = showOptions.teleport || 'body';
    position.value = { x: showOptions.x, y: showOptions.y };
    options.value = showOptions.options;

    nextTick(() => (visible.value = true));
  };

  const hide = () => {
    visible.value = false;
  };

  return {
    visible,
    teleport,
    position,
    options,

    show,
    hide,
  };
});
