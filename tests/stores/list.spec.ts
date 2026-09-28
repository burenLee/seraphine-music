import { beforeEach, describe, expect, it } from 'vitest';

import { defaultInfo, useListStore } from '@/stores/list';
import { useMusicStore } from '@/stores/music';
import { ListType, SortOrder, SortType } from '@/utils/params';

import { createTestPinia, makeListMusic, makeMusicList, makeSortInfo } from '../helpers/factories';

beforeEach(() => {
  createTestPinia();
});

describe('初始默认值', () => {
  it('四个列表与状态字段均为初始值', () => {
    const store = useListStore();

    expect(store.local.info).toStrictEqual({ ...defaultInfo, id: 'local', title: '本地歌曲' });
    expect(store.like.info).toStrictEqual({ ...defaultInfo });
    expect(store.play.list).toStrictEqual([]);
    expect(store.isHeaderLoading).toBe(true);
    expect(store.isTableLoading).toBe(true);
    expect(store.isInfiniting).toBe(false);
    expect(store.isChecking).toBe(false);
    expect(store.checkedList).toStrictEqual([]);
    expect(store.searchList).toBeUndefined();
    expect(store.sortMap).toStrictEqual({});
  });
});

describe('列表增删', () => {
  it('addList 去重追加并返回新增数量', () => {
    const store = useListStore();
    const m1 = makeListMusic({ id: 1, sort: 1 });
    const m2 = makeListMusic({ id: 2, sort: 2 });

    expect(store.addList(ListType.Play, [m1, m2])).toBe(2);
    expect(store.addList(ListType.Play, [m1])).toBe(0);
    expect(store.play.list).toHaveLength(2);
    expect(store.play.info.count).toBe(2);
  });

  it('addList 空数组返回 0', () => {
    const store = useListStore();
    expect(store.addList(ListType.Local, [])).toBe(0);
  });

  it('setList 整体替换', () => {
    const store = useListStore();
    const list = makeMusicList([makeListMusic()], { id: 'custom', title: '自定义' });

    store.setList(ListType.Show, list);

    expect(store.show.info.title).toBe('自定义');
    expect(store.show.list).toHaveLength(1);
  });

  it('removeList 移除并同步清理框选', () => {
    const store = useListStore();
    const m1 = makeListMusic({ id: 1 });
    store.addList(ListType.Play, [m1, makeListMusic({ id: 2 })]);
    store.toggleChecked();
    store.setCheckedList([1]);

    store.removeList(ListType.Play, [1]);

    expect(store.play.list).toHaveLength(1);
    expect(store.play.info.count).toBe(1);
    expect(store.checkedList).toStrictEqual([]);
  });

  it('removeList 空 id 守卫', () => {
    const store = useListStore();
    store.addList(ListType.Play, [makeListMusic()]);

    store.removeList(ListType.Play, ['']);

    expect(store.play.list).toHaveLength(1);
  });

  it('clearList 清空列表与框选', () => {
    const store = useListStore();
    store.addList(ListType.Play, [makeListMusic(), makeListMusic()]);

    store.clearList(ListType.Play);

    expect(store.play.list).toStrictEqual([]);
    expect(store.play.info.count).toBe(0);
  });

  it('resetList 重置为空白列表并复位框选状态', () => {
    const store = useListStore();
    store.addList(ListType.Show, [makeListMusic()]);
    store.toggleChecked();

    store.resetList(ListType.Show);

    expect(store.show.info).toStrictEqual(defaultInfo);
    expect(store.isChecking).toBe(false);
  });
});

describe('我喜欢列表', () => {
  it('增删清空', () => {
    const store = useListStore();
    const m1 = makeListMusic({ id: 1, fileId: 'f1' });
    const m2 = makeListMusic({ id: 2, fileId: 'f2' });

    store.addLikeList([m1, m2]);
    expect(store.like.list.get(1)).toBe('f1');
    expect(store.like.list.get(2)).toBe('f2');
    expect(store.like.list.size).toBe(2);

    store.removeLikeList([m1]);
    expect(store.like.list.has(1)).toBe(false);
    expect(store.like.list.size).toBe(1);

    store.clearLikeList();
    expect(store.like.list.size).toBe(0);
    expect(store.like.info.count).toBe(0);
  });
});

describe('addNextList 下一首播放', () => {
  it('插入到指定位置之后且 id 带唯一后缀', () => {
    const store = useListStore();
    const musicStore = useMusicStore();
    const m1 = makeListMusic({ id: 1 });
    const m2 = makeListMusic({ id: 2 });
    store.addList(ListType.Play, [m1, m2]);
    musicStore.music = m1;
    // 与已有歌曲 id 冲突, 触发加唯一后缀
    const insert = makeListMusic({ id: 1, title: '重复歌曲' });

    store.addNextPlay(insert);

    expect(store.play.list).toHaveLength(3);
    expect(store.play.list[1].id).toMatch(/^1 - /);
    expect(store.play.list[1].title).toBe('重复歌曲');
    expect(store.play.info.count).toBe(3);
  });
});

describe('框选', () => {
  it('toggleChecked 切换状态并清空选择', () => {
    const store = useListStore();
    store.toggleChecked();
    store.setCheckedList([1, 2]);
    expect(store.isChecking).toBe(true);
    expect(store.checkedList).toStrictEqual([1, 2]);

    store.toggleChecked();
    expect(store.isChecking).toBe(false);
    expect(store.checkedList).toStrictEqual([]);
  });

  it('setChecked / handleChecked 在非框选状态下无效', () => {
    const store = useListStore();

    store.setCheckedList([1]);
    store.handleChecked(2);

    expect(store.checkedList).toStrictEqual([]);
  });

  it('handleChecked 切换选中/取消', () => {
    const store = useListStore();
    store.toggleChecked();

    store.handleChecked(1);
    store.handleChecked(2);
    expect(store.checkedList).toStrictEqual([1, 2]);

    store.handleChecked(1);
    expect(store.checkedList).toStrictEqual([2]);
  });

  it('removeCheckedList 批量删除框选项', () => {
    const store = useListStore();
    store.addList(ListType.Play, [
      makeListMusic({ id: 1 }),
      makeListMusic({ id: 2 }),
      makeListMusic({ id: 3 }),
    ]);
    store.toggleChecked();
    store.setCheckedList([1, 3]);

    store.removeListChecked(ListType.Play);

    expect(store.play.list.map((m) => m.id)).toStrictEqual([2]);
    expect(store.play.info.count).toBe(1);
    expect(store.checkedList).toStrictEqual([]);
  });
});

describe('搜索', () => {
  it('按 title/artist/album 模糊匹配', () => {
    const store = useListStore();
    store.addList(ListType.Local, [
      makeListMusic({ id: 1, title: '晴天', artist: '周杰伦', album: '叶惠美' }),
      makeListMusic({ id: 2, title: 'Rain', artist: 'John', album: 'Blue' }),
    ]);

    store.search(ListType.Local, '晴天');
    expect(store.searchList?.map((m) => m.id)).toStrictEqual([1]);

    store.search(ListType.Local, 'John');
    expect(store.searchList?.map((m) => m.id)).toStrictEqual([2]);

    store.search(ListType.Local, 'Blue');
    expect(store.searchList?.map((m) => m.id)).toStrictEqual([2]);
  });

  it('空查询清空结果', () => {
    const store = useListStore();
    store.addList(ListType.Local, [makeListMusic()]);

    store.search(ListType.Local, 'x');
    store.search(ListType.Local, '');

    expect(store.searchList).toBeUndefined();

    store.search(ListType.Local, 'x');
    store.clearSearch();
    expect(store.searchList).toBeUndefined();
  });
});

describe('排序', () => {
  it('本地列表默认按 sort 字段倒序', () => {
    const store = useListStore();
    store.addList(ListType.Local, [
      makeListMusic({ id: 1, sort: 1 }),
      makeListMusic({ id: 2, sort: 3 }),
      makeListMusic({ id: 3, sort: 2 }),
    ]);

    expect(store.local.list.map((m) => m.sort)).toStrictEqual([3, 2, 1]);
  });

  it('setSortMap 后 addList 按标题升序', () => {
    const store = useListStore();
    // sortMap 的 key 是列表 info.id（本地列表固定为 'local'）
    store.setSortMap('local', makeSortInfo({ type: SortType.Title, order: SortOrder.ASC }));

    store.addList(ListType.Local, [
      makeListMusic({ id: 1, title: 'Charlie' }),
      makeListMusic({ id: 2, title: 'Alpha' }),
      makeListMusic({ id: 3, title: 'Bravo' }),
    ]);

    expect(store.local.list.map((m) => m.title)).toStrictEqual(['Alpha', 'Bravo', 'Charlie']);
  });

  it('时长倒序排序', () => {
    const store = useListStore();
    store.setSortMap('local', makeSortInfo({ type: SortType.Duration, order: SortOrder.DESC }));

    store.addList(ListType.Local, [
      makeListMusic({ id: 1, duration: 100 }),
      makeListMusic({ id: 2, duration: 300 }),
      makeListMusic({ id: 3, duration: 200 }),
    ]);

    expect(store.local.list.map((m) => m.duration)).toStrictEqual([300, 200, 100]);
  });
});
