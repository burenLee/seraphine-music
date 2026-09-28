import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import type { Directive } from 'vue';

import { notify } from '@/components/Notification.vue';

interface CopyEl extends HTMLElement {
  __copyHandler?: () => Promise<void>;
}

const vCopy: Directive<CopyEl, string> = {
  mounted(el, binding) {
    const copy = async () => {
      if (!binding.value) return;

      try {
        await writeText(String(binding.value));
        notify.success('复制成功');
      } catch {
        notify.error('复制失败');
      }
    };

    el.__copyHandler = copy;
    el.addEventListener('click', copy);
  },

  unmounted(el) {
    if (el.__copyHandler) {
      el.removeEventListener('click', el.__copyHandler);
      delete el.__copyHandler;
    }
  },
};

export default vCopy;
