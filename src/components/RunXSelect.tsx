import * as React from "react";

import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";

export function RunXSelect(props: {
  value: string;
  onChange: (value: string) => void;
  items: { value: string; label: string }[];
  placeholder: string;
}) {
  return (
    <Select value={props.value} onValueChange={props.onChange}>
      <SelectTrigger className="w-[180px] h-[30px]">
        <SelectValue placeholder={props.placeholder} />
      </SelectTrigger>
      <SelectContent>
        {props.items.map((item) => (
          <SelectItem key={item.value} value={item.value}>{item.label}</SelectItem>
        ))}
      </SelectContent>
    </Select>
  );
}
