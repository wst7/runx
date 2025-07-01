import useWebviewWindow from "./useWebviewWindow";

export default function useSettingsWebview() {
  const { openWindow, instance } = useWebviewWindow('settings', {
    title: "Settings",
    url: '/settings',
    maximized: false,
    minimizable: false,
    resizable: false
  });
  return {
    instance,
    openWindow,
  }
}