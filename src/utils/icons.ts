import Close from '~icons/custom-icons/Close'
import Maximize from '~icons/custom-icons/Maximize'
import Minimize from '~icons/custom-icons/Minimize'
import Minus from '~icons/custom-icons/Minus'
import Plus from '~icons/custom-icons/Plus'
import Restore from '~icons/custom-icons/Restore'
import AddFolder from '~icons/solar/add-folder-linear'
import Add from '~icons/solar/add-square-linear'
import AlignLeft from '~icons/solar/align-left-linear'
import AlignRight from '~icons/solar/align-right-linear'
import AlignCenter from '~icons/solar/align-vertical-spacing-linear'
import Down from '~icons/solar/alt-arrow-down-linear'
import Left from '~icons/solar/alt-arrow-left-linear'
import Right from '~icons/solar/alt-arrow-right-linear'
import Up from '~icons/solar/alt-arrow-up-linear'
import Chat from '~icons/solar/chat-dots-linear'
import CheckList from '~icons/solar/checklist-linear'
import CircleBottomDown from '~icons/solar/circle-bottom-down-linear'
import Timer from '~icons/solar/clock-circle-linear'
import DoubleDown from '~icons/solar/double-alt-arrow-down-linear'
import DoubleLeft from '~icons/solar/double-alt-arrow-left-linear'
import DoubleRight from '~icons/solar/double-alt-arrow-right-linear'
import DoubleUp from '~icons/solar/double-alt-arrow-up-linear'
import Download from '~icons/solar/download-linear'
import Exit from '~icons/solar/exit-linear'
import EyeClosed from '~icons/solar/eye-closed-linear'
import Eye from '~icons/solar/eye-linear'
import Filter from '~icons/solar/filter-linear'
import Folder from '~icons/solar/folder-linear'
import FullScreen from '~icons/solar/full-screen-linear'
import Picture from '~icons/solar/gallery-linear'
import Menu from '~icons/solar/hamburger-menu-linear'
import HeartBold from '~icons/solar/heart-angle-bold'
import Heart from '~icons/solar/heart-angle-linear'
import Info from '~icons/solar/info-circle-linear'
import Laptop from '~icons/solar/laptop-minimalistic-linear'
import Link from '~icons/solar/link-linear'
import Downloadlist from '~icons/solar/list-arrow-down-linear'
import LockBold from '~icons/solar/lock-bold'
import UnlockBold from '~icons/solar/lock-unlocked-bold'
import Logout from '~icons/solar/logout-linear'
import Search from '~icons/solar/magnifer-linear'
import ZoomInBold from '~icons/solar/magnifer-zoom-in-bold'
import ZoomOutBold from '~icons/solar/magnifer-zoom-out-bold'
import More from '~icons/solar/menu-dots-circle-linear'
import Mic from '~icons/solar/microphone-linear'
import ZoomIn from '~icons/solar/minimalistic-magnifer-zoom-in-linear'
import ZoomOut from '~icons/solar/minimalistic-magnifer-zoom-out-linear'
import Moon from '~icons/solar/moon-linear'
import ForwardLeftBold from '~icons/solar/multiple-forward-left-bold'
import ForwardRight from '~icons/solar/multiple-forward-left-linear'
import ForwardRightBold from '~icons/solar/multiple-forward-right-bold'
import ForwardLeft from '~icons/solar/multiple-forward-right-linear'
import AddMusic from '~icons/solar/music-library-2-linear'
import Music from '~icons/solar/music-note-2-linear'
import MusicBold from '~icons/solar/music-note-bold'
import Title from '~icons/solar/music-note-linear'
import LinesRemove from '~icons/solar/notification-lines-remove-linear'
import PauseBold from '~icons/solar/pause-bold'
import Pen from '~icons/solar/pen-linear'
import PIP from '~icons/solar/pip-linear'
import PlayBold from '~icons/solar/play-bold'
import Play from '~icons/solar/play-linear'
import Playlist from '~icons/solar/playlist-linear'
import Musiclist from '~icons/solar/playlist-minimalistic-2-linear'
import QuitFullScreen from '~icons/solar/quit-full-screen-linear'
import Refresh from '~icons/solar/refresh-linear'
import OrderPlay from '~icons/solar/repeat-bold-duotone'
import RepeatAll from '~icons/solar/repeat-linear'
import SinglePlay from '~icons/solar/repeat-one-line-duotone'
import RepeatOne from '~icons/solar/repeat-one-linear'
import Restart from '~icons/solar/restart-bold'
import SettingBold from '~icons/solar/settings-bold'
import Setting from '~icons/solar/settings-linear'
import Share from '~icons/solar/share-linear'
import RandomPlay from '~icons/solar/shuffle-linear'
import NextBold from '~icons/solar/skip-next-bold'
import PreviousBold from '~icons/solar/skip-previous-bold'
import Phone from '~icons/solar/smartphone-linear'
import SortDown from '~icons/solar/sort-from-bottom-to-top-linear'
import SortUp from '~icons/solar/sort-from-top-to-bottom-linear'
import Sun from '~icons/solar/sun-linear'
import Apparel from '~icons/solar/t-shirt-linear'
import Target from '~icons/solar/target-linear'
import Sort from '~icons/solar/transfer-vertical-linear'
import Bin from '~icons/solar/trash-bin-trash-linear'
import UnreadBold from '~icons/solar/unread-bold'
import Unread from '~icons/solar/unread-linear'
import User from '~icons/solar/user-hands-linear'
import Verified from '~icons/solar/verified-check-linear'
import Album from '~icons/solar/vinyl-record-linear'
import VolumeOff from '~icons/solar/volume-cross-linear'
import VolumeLoud from '~icons/solar/volume-loud-linear'
import VolumeSmall from '~icons/solar/volume-small-linear'
import Ring from '~icons/svg-spinners/ring-resize'

// 图标名称类型
export type IconName = keyof typeof IconMap

// 图标映射对象
export const IconMap = {
  // UI 控制相关图标
  Close,
  Maximize,
  Minimize,
  Restore,
  Plus,
  Minus,

  // Solar 图标库 - 导航与操作
  PIP,
  Setting,
  Refresh,
  More,
  Add,
  AddFolder,
  AddMusic,
  Filter,
  Sort,
  SortUp,
  SortDown,
  CheckList,

  // Solar 图标库 - 方向箭头
  Up,
  Down,
  Left,
  Right,
  DoubleUp,
  DoubleDown,
  DoubleLeft,
  DoubleRight,

  // Solar 图标库 - 音乐播放控制
  Play,
  PreviousBold,
  NextBold,
  PlayBold,
  PauseBold,
  RepeatAll,
  RepeatOne,
  OrderPlay,
  SinglePlay,
  RandomPlay,

  // Solar 图标库 - 音量控制
  VolumeLoud,
  VolumeSmall,
  VolumeOff,

  // Solar 图标库 - 播放列表与音乐
  Musiclist,
  Playlist,
  Downloadlist,
  Music,
  Album,
  Title,

  // Solar 图标库 - 其他功能图标
  Apparel,
  Chat,
  Mic,
  Search,
  Heart,
  HeartBold,
  Download,
  User,
  Timer,
  Picture,
  FullScreen,
  QuitFullScreen,
  Target,
  LinesRemove,
  Laptop,
  Phone,
  Unread,
  UnreadBold,
  Bin,
  AlignLeft,
  AlignRight,
  AlignCenter,
  Sun,
  Moon,
  ZoomIn,
  ZoomOut,
  Logout,
  Info,
  Share,
  Eye,
  EyeClosed,
  Pen,
  CircleBottomDown,
  Link,
  Folder,
  ForwardLeftBold,
  ForwardRightBold,
  ForwardLeft,
  ForwardRight,
  SettingBold,
  LockBold,
  UnlockBold,
  ZoomInBold,
  ZoomOutBold,
  MusicBold,
  Menu,
  Restart,
  Exit,
  Verified,

  Ring
}
