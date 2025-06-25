
import { LogOutput, LogOutputValueType } from "../LogType";

export default function LogItem(props: { value: LogOutput }) {
  return <LogValue value={props.value} />;
}
const LogValue = ({ value }: { value: LogOutput }) => {
  switch (value.type) {
    case LogOutputValueType.Null:
      return (
        <span className={`font-mono break-words text-[#F92672]`}>{"null"}</span>
      );
    case LogOutputValueType.Undefined:
      return (
        <span className={`font-mono break-words text-[#F92672]`}>
          {"undefined"}
        </span>
      );
    case LogOutputValueType.String:
      return (
        <span className={`font-mono break-words text-[#E6DB74]`}>
          {JSON.stringify(value.value)}
        </span>
      );
    case LogOutputValueType.Number:
      return (
        <span className={`font-mono break-words text-[#AE81FF]`}>
          {value.value}
        </span>
      );
    case LogOutputValueType.Boolean:
      return (
        <span className={`font-mono break-words text-[#A6E22E]`}>
          {value.value ? "true" : "false"}
        </span>
      );
    case LogOutputValueType.BigInt:
      return (
        <span className={`font-mono break-words text-[#AE81FF]`}>
          {`${value.value}n`}
        </span>
      );
    case LogOutputValueType.Date:
      return (
        <span className={`font-mono break-words text-[#66D9EF]`}>
          {value.value}
        </span>
      );
    case LogOutputValueType.Array:
      return (
        <span className="font-mono break-words">
          <span>{`[ `}</span>
          {value.value.map((item: LogOutput, i: number) => {
            return (
              <span key={i}>
                <LogValue value={item} />
                <span>{`${i === value.value.length - 1 ? "" : ", "}`}</span>
              </span>
            );
          })}
          <span>{` ]`}</span>
        </span>
      );
    case LogOutputValueType.Object:
      return (
        <span className="font-mono break-words">
          <span>{`{ `}</span>
          {Object.keys(value.value).map((key, i) => {
            return (
              <span key={key}>
                <span className="font-mono break-words text-[#78DCE8]">
                  {key}
                </span>
                <span className="font-mono break-words text-[#F9F871]">: </span>
                <LogValue value={value.value[key]} />
                <span>{`${
                  i == Object.keys(value.value).length - 1 ? "" : ", "
                }`}</span>
              </span>
            );
          })}
          <span>{` }`}</span>
        </span>
      );
    default:
      return (
        <span className={`font-mono break-words text-[#F92672]`}>
          {`Unknown type: ${value.type}`}
        </span>
      );
  }
};
