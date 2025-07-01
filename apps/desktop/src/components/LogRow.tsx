import { LogOutput } from "../LogType";
import LogItem from "./LogItem";



export default function LogRow(props: { value: LogOutput[] }) {
  const { value } = props;
  return (
    <div className="flex flex-row gap-x-4 flex-wrap">
      {value.map((item, idx) => (
        <LogItem key={idx} value={item} />
      ))}
    </div>
  );
}
