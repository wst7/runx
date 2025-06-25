import { LogOutput } from "../LogType";
import LogRow from "./LogRow";




export default function Output(props: { logs: LogOutput[][] }) {
  const { logs } = props;
  return (
    <div className="flex flex-col gap-y-4">
      {logs.map((item, idx) => (
        <LogRow key={idx} value={item} />
      ))}
    </div>
  );
}