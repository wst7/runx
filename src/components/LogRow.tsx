import LogItem from "./LogItem";

export type LogRowType = any[]


export default function LogRow(props: { value: LogRowType }) {
  const { value } = props;
  return (
    <div className="">
      {value.map((item, idx) => (
        <LogItem key={idx} value={item} />
      ))}
    </div>
  );
}
