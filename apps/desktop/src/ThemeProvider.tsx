import { Theme } from "@radix-ui/themes";
import "@radix-ui/themes/styles.css";
import { PropsWithChildren } from "react";

export default function ThemeWrap(props: PropsWithChildren) {
  const { children } = props;
  return <Theme>{children}</Theme>;
}
