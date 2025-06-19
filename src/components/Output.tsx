import LogRow, { LogRowType } from "./LogRow";




export default function Output(props: { logs: LogRowType[] }) {
  const { logs } = props;
  return (
    <div className="flex flex-col gap-y-4">
      {logs.map((item, idx) => (
        <LogRow key={idx} value={item} />
      ))}
    </div>
  );
}