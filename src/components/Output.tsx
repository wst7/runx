import LogRow, { LogRowType } from "./LogRow";




export default function Output(props: { logs: LogRowType[] }) {
  const { logs } = props;
  return (
    <div>
      {logs.map((item, idx) => (
        <LogRow key={idx} value={item} />
      ))}
    </div>
  );
}