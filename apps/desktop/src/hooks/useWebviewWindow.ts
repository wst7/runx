import { WebviewLabel, WebviewOptions } from "@tauri-apps/api/webview";
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
import { WindowOptions } from "@tauri-apps/api/window";
import { useCallback, useEffect, useRef, useState } from "react";

type UseWebviewWindowOptions = Omit<WebviewOptions, "x" | "y" | "width" | "height"> & WindowOptions;

export default function useWebviewWindow(
  label: WebviewLabel,
  options?: UseWebviewWindowOptions
) {
  const windowRef = useRef<WebviewWindow | null>(null);

  const openWindow = useCallback(async () => {
    const existing = await WebviewWindow.getByLabel(label);
    if (existing) {
      existing.show();
      existing.setFocus();
      windowRef.current = existing;
    } else {
      const win = new WebviewWindow(label, options);
      windowRef.current = win;

      win.once("tauri://created", () => {
        console.log(`窗口 ${label} 已创建`);
      });

      win.once("tauri://close-requested", async () => {
        console.log(`窗口 ${label} 请求关闭`);
        await win.close();
      });
    }
  }, [label, options]);




  return {
    openWindow,
    instance: windowRef.current,
  };

}
