import { defineStore } from 'pinia';
import { nextTick, ref } from 'vue';

/** 容器宽度 */
export const CONTAINER_WIDTH = 176;
/** 容器内边距 */
export const CONTAINER_PADDING = 4;
/** 分割线高度 */
export const DIVIDER_LINE_HEIGHT = 1;
/** 分割线分隔 */
export const DIVIDER_MARGIN = 8;
/** 分割线总高度 */
export const DIVIDER_Height = DIVIDER_LINE_HEIGHT + DIVIDER_MARGIN * 2;
/** 行高度 */
export const ROW_HEIGHT = 32;
/** 行宽度 */
export const ROW_WIDTH = CONTAINER_WIDTH - CONTAINER_PADDING * 2;

/** 累加选项高度得到容器总高度（含内边距） */
export const calcContainerHeight = (options: ContextMenuOption[]) => {
  return (
    options.reduce((h, option) => h + (option.divider ? DIVIDER_Height : ROW_HEIGHT), 0) +
    CONTAINER_PADDING * 2
  );
};

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
