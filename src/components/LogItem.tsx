import { useMemo } from "react";
import { getType } from "../utils";
enum ValueType {
  Null = "null",
  Undefined = "undefined",
  String = "string",
  Number = "number",
  Boolean = "boolean",
  Object = "object",
  Array = "array",
}

export default function LogItem(props: { value: any }) {
  return <LogValue value={props.value} />;
}
const LogValue = ({ value }: { value: any }) => {
  const valueType = useMemo<ValueType>(
    () => getType(value) as ValueType,
    [value]
  );
  if (valueType === ValueType.Null) {
    return (
      <span className={`font-mono break-words text-[#F92672]`}>{"null"}</span>
    );
  }
  if (valueType === ValueType.Undefined) {
    return (
      <span className={`font-mono break-words text-[#F92672]`}>
        {"undefined"}
      </span>
    );
  }
  if (valueType === ValueType.String) {
    return (
      <span className={`font-mono break-words text-[#E6DB74]`}>
        {JSON.stringify(value)}
      </span>
    );
  }
  if (valueType === ValueType.Number) {
    return (
      <span className={`font-mono break-words text-[#AE81FF]`}>{value}</span>
    );
  }
  if (valueType === ValueType.Boolean) {
    return (
      <span className={`font-mono break-words text-[#A6E22E]`}>
        {`${value}`}{" "}
      </span>
    );
  }
  if (valueType === ValueType.Object) {
    return (
      <span className="font-mono break-words">
        <span>{`{ `}</span>
        {Object.keys(value).map((key, i) => {
          return (
            <span key={key}>
              <span className="font-mono break-words text-[#78DCE8]">
                {key}
              </span>
              <span className="font-mono break-words text-[#F9F871]">: </span>
              <LogValue value={value[key]} />
              <span>{`${i == Object.keys(value).length - 1 ? "" : ", "}`}</span>
            </span>
          );
        })}
        <span>{` }`}</span>
      </span>
    );
  }
  if (valueType === ValueType.Array) {
    return (
      <span className="font-mono break-words">
        <span>{`[ `}</span>
        {value.map((item: any, i: number) => {
          return (
            <span key={i}>
              <LogValue value={item} />
              <span>{`${i == value.length - 1 ? "" : ", "}`}</span>
            </span>
          );
        })}
        <span>{` ]`}</span>
      </span>
    );
  }
};
