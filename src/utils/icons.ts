import Add from '~icons/material-symbols/add-rounded';
import Restore from '~icons/material-symbols/chrome-restore-outline-rounded';
import Close from '~icons/material-symbols/close-rounded';
import Remove from '~icons/material-symbols/remove-rounded';
import Square from '~icons/material-symbols/square-outline-rounded';
import AddFolder from '~icons/solar/add-folder-linear';
import AlignLeft from '~icons/solar/align-left-linear';
import AlignRight from '~icons/solar/align-right-linear';
import AlignCenter from '~icons/solar/align-vertical-spacing-linear';
import Down from '~icons/solar/alt-arrow-down-linear';
import Left from '~icons/solar/alt-arrow-left-linear';
import Right from '~icons/solar/alt-arrow-right-linear';
import Up from '~icons/solar/alt-arrow-up-linear';
import CheckList from '~icons/solar/checklist-linear';
import Timer from '~icons/solar/clock-circle-linear';
import DoubleDown from '~icons/solar/double-alt-arrow-down-linear';
import DoubleUp from '~icons/solar/double-alt-arrow-up-linear';
import Download from '~icons/solar/download-linear';
import Exit from '~icons/solar/exit-linear';
import EyeClosed from '~icons/solar/eye-closed-linear';
import Eye from '~icons/solar/eye-linear';
import Filter from '~icons/solar/filter-linear';
import Folder from '~icons/solar/folder-linear';
import FullScreen from '~icons/solar/full-screen-linear';
import Picture from '~icons/solar/gallery-linear';
import Menu from '~icons/solar/hamburger-menu-linear';
import HeartBold from '~icons/solar/heart-angle-bold';
import Heart from '~icons/solar/heart-angle-linear';
import Info from '~icons/solar/info-circle-linear';
import Laptop from '~icons/solar/laptop-minimalistic-linear';
import Link from '~icons/solar/link-linear';
import Lock from '~icons/solar/lock-linear';
import Logout from '~icons/solar/logout-linear';
import Search from '~icons/solar/magnifer-linear';
import More from '~icons/solar/menu-dots-square-linear';
import ZoomIn from '~icons/solar/minimalistic-magnifer-zoom-in-linear';
import ZoomOut from '~icons/solar/minimalistic-magnifer-zoom-out-linear';
import Moon from '~icons/solar/moon-linear';
import ForwardLeft from '~icons/solar/multiple-forward-left-linear';
import ForwardRight from '~icons/solar/multiple-forward-right-linear';
import MusicLibrary from '~icons/solar/music-library-2-linear';
import Music from '~icons/solar/music-note-2-linear';
import Empty from '~icons/solar/notification-lines-remove-linear';
import PauseBold from '~icons/solar/pause-bold';
import Pause from '~icons/solar/pause-linear';
import Pen from '~icons/solar/pen-linear';
import PIP from '~icons/solar/pip-linear';
import PlayBold from '~icons/solar/play-bold';
import Play from '~icons/solar/play-linear';
import Playlist from '~icons/solar/playlist-linear';
import QuitFullScreen from '~icons/solar/quit-full-screen-linear';
import Refresh from '~icons/solar/refresh-linear';
import OrderPlay from '~icons/solar/repeat-bold-duotone';
import RepeatAll from '~icons/solar/repeat-linear';
import SinglePlay from '~icons/solar/repeat-one-line-duotone';
import RepeatOne from '~icons/solar/repeat-one-linear';
import Restart from '~icons/solar/restart-bold';
import Setting from '~icons/solar/settings-linear';
import Share from '~icons/solar/share-linear';
import RandomPlay from '~icons/solar/shuffle-linear';
import NextBold from '~icons/solar/skip-next-bold';
import Next from '~icons/solar/skip-next-linear';
import PreviousBold from '~icons/solar/skip-previous-bold';
import Previous from '~icons/solar/skip-previous-linear';
import SortDown from '~icons/solar/sort-from-bottom-to-top-linear';
import SortUp from '~icons/solar/sort-from-top-to-bottom-linear';
import Sun from '~icons/solar/sun-linear';
import Target from '~icons/solar/target-linear';
import Sort from '~icons/solar/transfer-vertical-linear';
import Bin from '~icons/solar/trash-bin-trash-linear';
import UnreadBold from '~icons/solar/unread-bold';
import Unread from '~icons/solar/unread-linear';
import User from '~icons/solar/user-hands-linear';
import Verified from '~icons/solar/verified-check-linear';
import Album from '~icons/solar/vinyl-record-linear';
import VolumeOff from '~icons/solar/volume-cross-linear';
import VolumeLoud from '~icons/solar/volume-loud-linear';
import VolumeSmall from '~icons/solar/volume-small-linear';
import Ring from '~icons/svg-spinners/ring-resize';

export const IconMap = {
  Add,
  Restore,
  Close,
  Remove,
  Square,
  AddFolder,
  AlignLeft,
  AlignRight,
  AlignCenter,
  Down,
  Left,
  Right,
  Up,
  CheckList,
  Timer,
  DoubleDown,
  DoubleUp,
  Download,
  Exit,
  EyeClosed,
  Eye,
  Filter,
  Folder,
  FullScreen,
  Picture,
  Menu,
  HeartBold,
  Heart,
  Info,
  Laptop,
  Link,
  Lock,
  Logout,
  Search,
  More,
  ZoomIn,
  ZoomOut,
  Moon,
  ForwardLeft,
  ForwardRight,
  MusicLibrary,
  Music,
  Empty,
  PauseBold,
  Pause,
  Pen,
  PIP,
  PlayBold,
  Play,
  Playlist,
  QuitFullScreen,
  Refresh,
  OrderPlay,
  RepeatAll,
  SinglePlay,
  RepeatOne,
  Restart,
  Setting,
  Share,
  RandomPlay,
  NextBold,
  Next,
  PreviousBold,
  Previous,
  SortDown,
  SortUp,
  Sun,
  Target,
  Sort,
  Bin,
  UnreadBold,
  Unread,
  User,
  Verified,
  Album,
  VolumeOff,
  VolumeLoud,
  VolumeSmall,
  Ring,
} as const;

export type IconName = keyof typeof IconMap;
