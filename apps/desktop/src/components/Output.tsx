import { LogOutput } from "../LogType";
import LogRow from "./LogRow";


type OutputProps = {
  logs?: { language: string, value: LogOutput[][] }
}

export default function Output(props: OutputProps) {
  const { logs } = props;

  
  return (
    <div className="flex flex-col gap-y-4">
      {logs?.value?.map((item, idx) => (
        <LogRow key={idx} value={item} />
      ))}
    </div>
  );
}